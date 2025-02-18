use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{collections::HashMap, sync::Mutex, vec};
use tauri::State;
use reqwest;
use futures::StreamExt;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::env;  // Add this for getting home directory

#[derive(Clone,Serialize,Deserialize)]
struct Port {
    service: String,
    application: String,
    protocol: String,
    number: u16,
    state: String,
    data: Vec<PortNotes>,
}

#[derive(Clone,Serialize,Deserialize)]
enum PortNotes {
    NmapScan(Vec<String>),
    Credentials {
        name: Option<String>,
        hash: Option<String>,
        password: Option<String>
    },
    PentestNote {
        stage: String,  // e.g., "Enumeration", "Exploitation", "Post-Exploitation"
        content: String,
        timestamp: String
    },
    None,
}

#[derive(Clone, Serialize, Deserialize)]
struct Machine {
    id: u32,
    hostname: String,
    icon: String,
    ip: String,
    ports: Vec<Port>,
}

#[derive(Clone, Serialize, Deserialize)]
struct JsMachine {
    id: u32,
    hostname: String,
    icon: String,
    ip: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct JsPort {
    service: String,
    number: u16,
    protocol: String,
    state: String,
    application: String,
    data: Vec<PortNotes>,  // Add details field to JsPort
}

#[derive(Clone, Serialize, Deserialize)]
struct JsWorkspace {
    name: String,
    id: u32,
    ip_range: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Workspace {
    name: String,
    id: u32,
    data: Vec<Machine>,
    ip_range: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Database {
    name: String,
    data: Vec<Workspace>,
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiPort {
    service: String,
    application: String,
    protocol: String,
    number: u16,
    state: String,
    data: Option<Vec<String>>, // Make data field optional
    details: Option<Vec<String>>,  // Add new field
}

#[derive(Clone, Serialize, Deserialize)]
struct NetworkScan {
    cidr: String,
    active_hosts: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ChatContext {
    type_: String,
    workspace_id: u32,
    machine_id: Option<u32>,
    port_number: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct ToolStatus {
    rustscan: bool,
    nmap: bool,
    ollama: bool,
}

#[cfg(target_os = "windows")]
fn get_data_dir() -> PathBuf {
    let app_data = env::var("APPDATA")
        .expect("Failed to get APPDATA directory");
    let mut path = PathBuf::from(app_data);
    path.push("NetVision");  // Your app name
    path.push("workspaces");
    fs::create_dir_all(&path).expect("Failed to create workspaces directory");
    path
}

#[cfg(target_os = "linux")]
fn get_data_dir() -> PathBuf {
    let home = env::var("HOME")
        .expect("Failed to get HOME directory");
    let mut path = PathBuf::from(home);
    path.push(".local");
    path.push("share");
    path.push("netvision");
    path.push("workspaces");
    fs::create_dir_all(&path).expect("Failed to create workspaces directory");
    path
}

#[cfg(target_os = "macos")]
fn get_data_dir() -> PathBuf {
    let home = env::var("HOME")
        .expect("Failed to get HOME directory");
    let mut path = PathBuf::from(home);
    path.push("Library");
    path.push("Application Support");
    path.push("NetVision");
    path.push("workspaces");
    fs::create_dir_all(&path).expect("Failed to create workspaces directory");
    path
}

fn load_database() -> Database {
    let path = get_data_dir().join("database.json");
    if path.exists() {
        let data = fs::read_to_string(path)
            .expect("Failed to read database file");
        serde_json::from_str(&data)
            .unwrap_or_else(|_| Database {
                name: "Default".to_string(),
                data: vec![],
            })
    } else {
        Database {
            name: "Default".to_string(),
            data: vec![],
        }
    }
}

fn save_database(database: &Database) -> Result<(), String> {
    let path = get_data_dir().join("database.json");
    let json = serde_json::to_string_pretty(&database)
        .map_err(|e| format!("Failed to serialize database: {}", e))?;
    fs::write(path, json)
        .map_err(|e| format!("Failed to write database file: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn scan_ip(ip: String) -> Result<String, String> {
    let url = format!("http://127.0.0.1:8084/scan/{}", ip);
    let response = reqwest::get(&url).await.map_err(|e| format!("Failed to call API: {}", e))?;
    
    let response_text = response.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
    println!("Response: {}", response_text);
    Ok(response_text)
}

#[tauri::command]
async fn scan_machine(database: State<'_, Mutex<Database>>, workspace_id: u32, machine_id: u32) -> Result<String, String> {
    println!("╔════ Starting Machine Scan ════");
    println!("║ Workspace ID: {}", workspace_id);
    println!("║ Machine ID: {}", machine_id);
    
    let machine_ip = {
        let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let machine = db.data.iter().find(|workspace| workspace.id == workspace_id)
            .and_then(|workspace| workspace.data.iter().find(|machine| machine.id == machine_id))
            .ok_or_else(|| format!("Machine '{}' not found in workspace '{}'", machine_id, workspace_id))?;
        println!("║ Found machine: {} ({})", machine.hostname, machine.ip);
        machine.ip.clone()
    };
    
    println!("║ Initiating port scan for IP: {}", machine_ip);
    let response_text = scan_ip(machine_ip).await?;
    println!("║ Raw scan response received: {}", response_text);

    if response_text.contains("N/A") {
        println!("║ No open ports found");
        let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let machine = db.data.iter_mut().find(|workspace| workspace.id == workspace_id)
            .and_then(|workspace| workspace.data.iter_mut().find(|machine| machine.id == machine_id))
            .ok_or_else(|| format!("Machine '{}' not found in workspace '{}'", machine_id, workspace_id))?;
        
        machine.ports = vec![];
        println!("╚════ Scan Complete - No Open Ports ════");
        return Ok("Scan completed. No open ports found.".to_string());
    }

    let api_ports: Vec<ApiPort> = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    println!("║ Parsed {} ports from response", api_ports.len());
    
    let ports: Vec<Port> = api_ports.clone().into_iter().map(|api_port| {
        let data = if let Some(details) = api_port.details {
            if !details.is_empty() {
                vec![PortNotes::NmapScan(details)]
            } else {
                vec![PortNotes::None]
            }
        } else {
            vec![PortNotes::None]
        };

        Port {
            service: api_port.service,
            application: api_port.application,
            protocol: api_port.protocol,
            number: api_port.number,
            state: api_port.state,
            data,
        }
    }).collect();
    
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let machine = db.data.iter_mut().find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter_mut().find(|machine| machine.id == machine_id))
        .ok_or_else(|| format!("Machine '{}' not found in workspace '{}'", machine_id, workspace_id))?;
    
    machine.ports = ports;
    println!("╚════ Scan Complete ════");
    Ok(format!("Scan completed successfully. Found {} ports", api_ports.len()))
}

#[tauri::command]
fn workspaces(database: State<Mutex<Database>>) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    println!("Loading workspaces, found {} workspaces", db.data.len()); // Debug log
    let workspaces: Vec<JsWorkspace> = db.data.iter().map(|workspace| {
        println!("Processing workspace: {} ({})", workspace.name, workspace.id); // Debug log
        JsWorkspace {
            id: workspace.id,
            name: workspace.name.clone(),
            ip_range: workspace.ip_range.clone(),
        }
    }).collect();
    
    let json = serde_json::to_string(&workspaces)
        .map_err(|e| format!("Failed to serialize workspaces: {}", e))?;
    println!("Serialized workspaces: {}", json); // Debug log
    Ok(json)
}

#[tauri::command]
fn machines(database: State<Mutex<Database>>, workspace_id: u32) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let machines: Vec<JsMachine> = db.data.iter().find(|workspace| workspace.id == workspace_id).map(|workspace| {
        workspace.data.iter().map(|machine| JsMachine {
            hostname: machine.hostname.clone(),
            icon: machine.icon.clone(),
            ip: machine.ip.clone(),
            id: machine.id.clone(),
        }).collect()
    }).unwrap_or_default();
    serde_json::to_string(&machines).map_err(|e| format!("Failed to serialize machines: {}", e))
}

#[tauri::command]
fn ports(database: State<Mutex<Database>>, workspace_id: u32, machine_id: u32) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let ports: Vec<JsPort> = db.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter().find(|machine| machine.id == machine_id))
        .map(|machine| {
            machine.ports.iter().map(|port| JsPort {
                service: port.service.clone(),
                number: port.number,
                protocol: port.protocol.clone(),
                state: port.state.clone(),
                application: port.application.clone(),
                data: port.data.clone(),
            }).collect()
        }).unwrap_or_default();
    serde_json::to_string(&ports).map_err(|e| format!("Failed to serialize ports: {}", e))
}

#[tauri::command]
fn get_machine(database: State<Mutex<Database>>, workspace_id: u32, machine_id: u32) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    db.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter().find(|machine| machine.id == machine_id))
        .map(|machine| JsMachine {
            id: machine.id,
            hostname: machine.hostname.clone(),
            icon: machine.icon.clone(),
            ip: machine.ip.clone(),
        })
        .map(|js_machine| serde_json::to_string(&js_machine).map_err(|e| format!("Failed to serialize machine: {}", e)))
        .unwrap_or_else(|| Err(format!("Machine '{}' not found in workspace '{}'", machine_id, workspace_id)))
}

#[tauri::command]
fn get_workspace(database: State<Mutex<Database>>, workspace_id: u32) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    db.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .map(|workspace| JsWorkspace {
            id: workspace.id,
            name: workspace.name.clone(),
            ip_range: workspace.ip_range.clone(),
        })
        .map(|js_workspace| serde_json::to_string(&js_workspace).map_err(|e| format!("Failed to serialize workspace: {}", e)))
        .unwrap_or_else(|| Err(format!("Workspace '{}' not found", workspace_id)))
}

#[tauri::command]
fn get_port(database: State<Mutex<Database>>, workspace_id: u32, machine_id: u32, port_number: u16) -> Result<String, String> {
    let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    db.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter().find(|machine| machine.id == machine_id))
        .and_then(|machine| machine.ports.iter().find(|port| port.number == port_number))
        .map(|port| Port {
            service: port.service.clone(),
            number: port.number,
            data: port.data.clone(),
            application: port.application.clone(),
            protocol: port.protocol.clone(),
            state: port.state.clone(),
        })
        .map(|js_port| serde_json::to_string(&js_port).map_err(|e| format!("Failed to serialize port: {}", e)))
        .unwrap_or_else(|| Err(format!("Port '{}' not found in machine '{}' in workspace '{}'", port_number, machine_id, workspace_id)))
}

#[tauri::command]
fn add_workspace(database: State<Mutex<Database>>, name: String, ip_range: String) -> Result<String, String> {
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let new_workspace = Workspace {
        name,
        id: db.data.len() as u32 + 1,
        data: vec![],
        ip_range,
    };
    db.data.push(new_workspace);
    
    // Save the updated database
    save_database(&db)?;
    
    Ok("Workspace added successfully".to_string())
}

#[tauri::command]
fn add_machine(database: State<Mutex<Database>>, workspace_id: u32, name: String, ip: String) -> Result<String, String> {
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let workspace = db.data.iter_mut().find(|workspace| workspace.id == workspace_id)
        .ok_or_else(|| format!("Workspace '{}' not found", workspace_id))?;
    
    let new_machine = Machine {
        id: workspace.data.len() as u32 + 1,
        hostname: name,
        icon: "PC".to_string(),
        ip,
        ports: vec![],
    };
    workspace.data.push(new_machine);
    
    save_database(&db)?;
    Ok("Machine added successfully".to_string())
}

#[tauri::command]
async fn discover_hosts(database: State<'_, Mutex<Database>>, workspace_id: u32) -> Result<String, String> {
    println!("╔════ Starting Network Discovery ════");
    println!("║ Workspace ID: {}", workspace_id);
    
    let ip_range = {
        let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let workspace = db.data.iter().find(|workspace| workspace.id == workspace_id)
            .ok_or_else(|| format!("Workspace '{}' not found", workspace_id))?;
        println!("║ Found workspace: {} ({})", workspace.name, workspace.ip_range);
        workspace.ip_range.clone()
    };

    println!("║ Scanning network range: {}", ip_range);
    let url = format!("http://127.0.0.1:8084/discover/{}", ip_range.replace("/", "-"));
    let response = reqwest::get(&url).await.map_err(|e| format!("Failed to call API: {}", e))?;
    println!("║ API response received");
    
    let network_scan: NetworkScan = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
    println!("║ Found {} active hosts", network_scan.active_hosts.len());
    
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let workspace = db.data.iter_mut().find(|workspace| workspace.id == workspace_id)
        .ok_or_else(|| format!("Workspace '{}' not found", workspace_id))?;
    
    println!("Adding discovered hosts to database");
    for (index, host) in network_scan.active_hosts.iter().enumerate() {
        println!("Adding host: {}", host);
        let new_machine = Machine {
            id: workspace.data.len() as u32 + 1 + index as u32,
            hostname: format!("Host_{}", host),
            icon: "PC".to_string(),
            ip: host.clone(),
            ports: vec![],
        };
        workspace.data.push(new_machine);
    }

    save_database(&db)?;
    println!("╚════ Network Discovery Complete ════");
    Ok(format!("Network scan completed. Found and added {} hosts", network_scan.active_hosts.len()))
}

#[tauri::command]
fn update_port_notes(
    database: State<Mutex<Database>>,
    workspace_id: u32,
    machine_id: u32,
    port_number: u16,
    notes: Vec<PortNotes>
) -> Result<String, String> {
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    let port = db.data.iter_mut()
        .find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter_mut().find(|machine| machine.id == machine_id))
        .and_then(|machine| machine.ports.iter_mut().find(|port| port.number == port_number))
        .ok_or_else(|| format!("Port not found"))?;

    port.data = notes;
    
    save_database(&db)?;
    Ok("Port notes updated successfully".to_string())
}

#[tauri::command(rename_all = "camelCase")]
fn update_note_content(
    database: State<Mutex<Database>>,
    workspaceId: u32,
    machineId: u32,
    portNumber: u16,
    noteIndex: usize,
    newContent: String,
) -> Result<String, String> {
    let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let port = db.data.iter_mut()
        .find(|ws| ws.id == workspaceId)
        .and_then(|ws| ws.data.iter_mut().find(|m| m.id == machineId))
        .and_then(|m| m.ports.iter_mut().find(|p| p.number == portNumber))
        .ok_or_else(|| "Port not found".to_string())?;

    if noteIndex >= port.data.len() {
        return Err("Note index out of bounds".into());
    }

    match &mut port.data[noteIndex] {
        PortNotes::PentestNote { content, .. } => {
            *content = newContent;
            save_database(&db)?;
            Ok("Note updated successfully".to_string())
        },
        _ => Err("Note at this index is not a PentestNote".into())
    }
}

#[tauri::command]
async fn ask_question<'a>(database: State<'a, Mutex<Database>>, question: String, context: ChatContext) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    // Get all relevant context data from the database
    let context_data = {
        let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let mut context_str = String::new();
        
        if let Some(workspace) = db.data.iter().find(|w| w.id == context.workspace_id) {
            context_str.push_str(&format!("Workspace: {}\n", workspace.name));
            
            if let Some(machine_id) = context.machine_id {
                if let Some(machine) = workspace.data.iter().find(|m| m.id == machine_id) {
                    context_str.push_str(&format!("Machine: {} ({})\n", machine.hostname, machine.ip));
                    
                    if let Some(port_number) = context.port_number {
                        if let Some(port) = machine.ports.iter().find(|p| p.number == port_number) {
                            context_str.push_str(&format!("Port {}/{} - {} ({})\n", 
                                port.number, port.protocol, port.service, port.application));
                            
                            // Add all port notes
                            for note in &port.data {
                                match note {
                                    PortNotes::NmapScan(details) => {
                                        context_str.push_str("Nmap Scan Results:\n");
                                        for detail in details {
                                            context_str.push_str(&format!("- {}\n", detail));
                                        }
                                    },
                                    PortNotes::Credentials { name, hash, password } => {
                                        context_str.push_str("Credentials Found:\n");
                                        if let Some(n) = name { context_str.push_str(&format!("- Username: {}\n", n)); }
                                        if let Some(h) = hash { context_str.push_str(&format!("- Hash: {}\n", h)); }
                                        if let Some(p) = password { context_str.push_str(&format!("- Password: {}\n", p)); }
                                    },
                                    PortNotes::PentestNote { stage, content, timestamp } => {
                                        context_str.push_str(&format!("Pentest Note ({} - {}):\n{}\n", stage, timestamp, content));
                                    },
                                    PortNotes::None => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        context_str
    };

    // Combine context data with the question
    let full_question = format!(
        "You are an expert penetration tester and security analyst. Focus on identifying vulnerabilities, \
        potential attack vectors, and security implications. Be direct and concise. \
        Provide practical security insights based on this context:\n\n{}\n\nQuestion: {}", 
        context_data, 
        question
    );

    let payload = json!({
        "question": full_question
    });

    let response = client.post("http://127.0.0.1:8084/ask")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call API: {}", e))?;

    let text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    Ok(text)
}

#[tauri::command]
async fn check_tools() -> Result<String, String> {
    let response = reqwest::get("http://127.0.0.1:8084/tools")
        .await
        .map_err(|e| format!("Failed to call API: {}", e))?;

    let text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    Ok(text)
}

#[tauri::command]
async fn analyze_port(database: State<'_, Mutex<Database>>, workspace_id: u32, machine_id: u32, port_number: u16) -> Result<String, String> {
    // Gather port info and create context in a separate scope
    let context = {
        let db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let port = db.data.iter()
            .find(|w| w.id == workspace_id)
            .and_then(|w| w.data.iter().find(|m| m.id == machine_id))
            .and_then(|m| m.ports.iter().find(|p| p.number == port_number))
            .ok_or_else(|| "Port not found".to_string())?;

        json!({
            "text": format!(
                "Analyze this port for security vulnerabilities:\n\
                Service: {}\n\
                Port: {}\n\
                Protocol: {}\n\
                State: {}\n\
                Application: {}\n\n\
                Scan Results:\n{}", 
                port.service,
                port.number,
                port.protocol,
                port.state,
                port.application,
                port.data.iter()
                    .filter_map(|note| match note {
                        PortNotes::NmapScan(details) => Some(details.join("\n")),
                        _ => None
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }).to_string()
    };

    // Make the API call
    let client = reqwest::Client::new();
    let payload = json!({
        "question": format!(
            "Security analysis of port scan.\n\
            Format in Markdown:\n\
            1. Critical Vulnerabilities\n\
            2. Exploitation Methods\n\
            3. Security Recommendations\n\n\
            {}", 
            context
        )
    });

    let response = client.post("http://127.0.0.1:8084/ask")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call AI API: {}", e))?;

    let analysis = response.text().await
        .map_err(|e| format!("Failed to read AI response: {}", e))?;

    println!("Received AI analysis, updating database...");
    // Update database
    {
        let mut db = database.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        let port = db.data.iter_mut()
            .find(|w| w.id == workspace_id)
            .and_then(|w| w.data.iter_mut().find(|m| m.id == machine_id))
            .and_then(|m| m.ports.iter_mut().find(|p| p.number == port_number))
            .ok_or_else(|| "Port not found".to_string())?;

        let ai_note = PortNotes::NmapScan(vec![
            "=== AI Security Analysis ===".to_string(),
            analysis
        ]);

        port.data.push(ai_note);
        save_database(&db)?;
    }

    println!("Analysis completed successfully");
    Ok("Analysis completed and saved".to_string())
}

pub fn run() {
    let database = load_database();

    tauri::Builder::default()
        .manage(Mutex::new(database))
        .invoke_handler(tauri::generate_handler![
            workspaces, machines, ports, get_machine, get_workspace, get_port, 
            scan_ip, scan_machine, add_workspace, add_machine, discover_hosts, 
            update_port_notes, update_note_content, ask_question, check_tools,
            analyze_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
