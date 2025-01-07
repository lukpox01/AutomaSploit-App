use serde::{Serialize, Deserialize};
use std::{collections::HashMap, sync::Mutex, vec};
use tauri::State;

#[derive(Clone,Serialize,Deserialize)]
struct Port {
    service: String,
    number: u16,
    data: Vec<String>,
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


#[tauri::command]
fn workspaces(database: State<Database>) -> Result<String, String> {
    let workspaces: Vec<JsWorkspace> = database.data.iter().map(|workspace| JsWorkspace {
        id: workspace.id,
        name: workspace.name.clone(),
        ip_range: workspace.ip_range.clone(),
    }).collect();
    serde_json::to_string(&workspaces).map_err(|e| format!("Failed to serialize workspaces: {}", e))
}

#[tauri::command]
fn machines(database: State<Database>, workspace_id: u32) -> Result<String, String> {
    let machines: Vec<JsMachine> = database.data.iter().find(|workspace| workspace.id == workspace_id).map(|workspace| {
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
fn ports(database: State<Database>, workspace_id: u32, machine_id: u32) -> Result<String, String> {
    let ports: Vec<JsPort> = database.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .and_then(|workspace| workspace.data.iter().find(|machine| machine.id == machine_id))
        .map(|machine| {
            machine.ports.iter().map(|port| JsPort {
                service: port.service.clone(),
                number: port.number,
            }).collect()
        }).unwrap_or_default();
    serde_json::to_string(&ports).map_err(|e| format!("Failed to serialize ports: {}", e))
}

#[tauri::command]
fn get_machine(database: State<Database>, workspace_id: u32, machine_id: u32) -> Result<String, String> {
    database.data.iter()
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
fn get_workspace(database: State<Database>, workspace_id: u32) -> Result<String, String> {
    database.data.iter()
        .find(|workspace| workspace.id == workspace_id)
        .map(|workspace| JsWorkspace {
            id: workspace.id,
            name: workspace.name.clone(),
            ip_range: workspace.ip_range.clone(),
        })
        .map(|js_workspace| serde_json::to_string(&js_workspace).map_err(|e| format!("Failed to serialize workspace: {}", e)))
        .unwrap_or_else(|| Err(format!("Workspace '{}' not found", workspace_id)))
}

pub fn run() {

    let database = Database {
        name: "Lukas".to_string(),

        data: vec![
            Workspace {        
                ip_range: "10.10.10.0/16".to_string(),
                id: 20,
            name: "School".to_string(),
            data: vec![
                Machine {
                    id:1,
                hostname: "PCSOSE1".to_string(),
                ip: "10.10.10.1".to_string(),
                icon: "PC".to_string(),
                ports: vec![
                    Port {
                    service: "HTTP".to_string(),
                    number: 80,
                    data: vec!["GET / HTTP/1.1".to_string()],
                    }, 
                    Port {
                    service: "HTTPS".to_string(),
                    number: 443,
                    data: vec!["GET / HTTP/1.1".to_string()],
                    },
                ],
            }],
        }],
    };


    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![workspaces, machines, ports, get_machine, get_workspace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
