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
enum PortNotes{
    NmapScan(Vec<String>),
    Credentials {
        name: Option<String>,
        hash: Option<String>,
        password: Option<String>
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
    let workspaces: Vec<JsWorkspace> = db.data.iter().map(|workspace| JsWorkspace {
        id: workspace.id,
        name: workspace.name.clone(),
        ip_range: workspace.ip_range.clone(),
    }).collect();
    serde_json::to_string(&workspaces).map_err(|e| format!("Failed to serialize workspaces: {}", e))
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

#[tauri::command]
async fn ask_question(question: String, context: ChatContext) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let mut context_data = serde_json::json!({
        "question": question,
        "context_type": context.type_,
        "workspace_id": context.workspace_id,
    });

    if let Some(machine_id) = context.machine_id {
        context_data["machine_id"] = json!(machine_id);
    }

    if let Some(port_number) = context.port_number {
        context_data["port_number"] = json!(port_number);
    }

    let response = client.post("http://127.0.0.1:8084/ask")
        .json(&context_data)
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

pub fn run() {
    let database = load_database();

    tauri::Builder::default()
        .manage(Mutex::new(database))
        .invoke_handler(tauri::generate_handler![
            workspaces, machines, ports, get_machine, get_workspace, get_port, 
            scan_ip, scan_machine, add_workspace, add_machine, discover_hosts, 
            update_port_notes, ask_question, check_tools
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
