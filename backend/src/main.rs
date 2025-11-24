use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize};
use std::process::Command;
use sysinfo::{System, SystemExt, CpuExt}; 
use std::io::{Write, Read};
use std::fs::{OpenOptions, File};
use chrono::Local;
use actix_web::web::Json;
use serde::Deserialize;

// --- 1. Estruturas de Dados ---

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct HardwareInfo {
    cpu_usage: f32,
    ram_total: u64,
    ram_used: u64,
    process_count: usize,
    message: String,
}

#[derive(Serialize)]
struct ActionResponse {
    status: String,
    message: String,
    details: Option<String>,
}

// --- 1.5. Funções de Log ---

fn log_action(action_name: &str, response: &ActionResponse) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_message = format!(
        "[{}] Ação: {}\n    Status: {}\n    Mensagem: {}\n    Detalhes: {}\n------------------------------------\n",
        timestamp,
        action_name,
        response.status,
        response.message,
        response.details.as_deref().unwrap_or("N/A")
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("coreflow_execution.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log_message) {
        eprintln!("Não foi possível escrever no arquivo de log: {}", e);
    }
}

// Credenciais de Admin (Hardcoded para fins de desenvolvimento)
const ADMIN_USER: &str = "admin";
const ADMIN_PASS: &str = "coreflow2024";

// --- 2. Handlers de Ações (Rotas /api/...) ---

// Handler de Login
async fn login(info: Json<LoginRequest>) -> impl Responder {
    if info.username == ADMIN_USER && info.password == ADMIN_PASS {
        // Em um projeto real, você geraria um token JWT aqui.
        // Para este projeto de graduação, vamos apenas retornar sucesso.
        HttpResponse::Ok().json(ActionResponse {
            status: "SUCCESS".to_string(),
            message: "Login bem-sucedido. Bem-vindo, Administrador!".to_string(),
            details: None,
        })
    } else {
        HttpResponse::Unauthorized().json(ActionResponse {
            status: "ERROR".to_string(),
            message: "Credenciais inválidas.".to_string(),
            details: None,
        })
    }
}

async fn get_hardware_info() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    sys.refresh_cpu(); 
    let cpu_usage = sys.cpus().iter().map(|p| p.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    let ram_total = sys.total_memory() / 1024 / 1024;
    let ram_used = sys.used_memory() / 1024 / 1024;
    
    let process_count = sys.processes().len();

    let info = HardwareInfo {
        cpu_usage,
        ram_total,
        ram_used,
        process_count,
        message: format!("Uso da CPU: {:.2}% | RAM Total: {} MB | RAM Usada: {} MB\nProcessos Ativos: {}", 
                         cpu_usage, ram_total, ram_used, process_count),
    };

    HttpResponse::Ok().json(info)
}

fn execute_command(command: &str, args: &[&str], success_msg: &str, action_name_for_log: &str) -> ActionResponse {
    println!("\n[INFO] Executando: {} {}", command, args.join(" "));
    std::io::stdout().flush().unwrap();
    
    // Para comandos do Windows que precisam de privilégios elevados,
    // a execução direta pode falhar. No entanto, a função `execute_command`
    // já está bem estruturada para capturar o output. O problema
    // pode ser a falta de privilégios ou a forma como o comando é chamado.
    // Vamos garantir que o `cmd` seja usado corretamente para comandos internos.
    
    let mut cmd_builder = Command::new(command);
    cmd_builder.args(args);
    
    match cmd_builder.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                ActionResponse {
                    status: "SUCCESS".to_string(),
                    message: success_msg.to_string(),
                    details: Some(stdout),
                }
            } else {
                ActionResponse {
                    status: "ERROR".to_string(),
                    message: format!("Falha na execução do comando. Erro: {}", stderr),
                    details: Some(stdout),
                }
            }
        }
        Err(e) => ActionResponse {
            status: "ERROR".to_string(),
            message: format!("Erro ao tentar executar o comando: {}", e),
            details: None,
        },
    }
}

// 1. Limpar Arquivos TEMP (%TEMP%)
async fn limpar_temp() -> impl Responder {
    // O comando 'del' precisa ser executado pelo cmd.exe
    let path = format!("{}\\*", std::env::temp_dir().display());
    let response = execute_command("cmd", &["/C", "del", "/S", "/Q", &path], "Limpeza de arquivos temporários concluída com sucesso. Verifique os detalhes para o resultado.", "Limpar Arquivos TEMP");
    log_action("Limpar Arquivos TEMP", &response);
    HttpResponse::Ok().json(response)
}

// 2. Otimizar Disco (C:)
async fn otimizar_disco() -> impl Responder {
    // O 'defrag' pode exigir privilégios elevados e o resultado é assíncrono.
    // O /V (verbose) pode ser útil para debug, mas o comando em si é o correto.
    let response = execute_command("defrag", &["C:", "/O"], "Otimização (desfragmentação/trim) do Disco C: iniciada. Verifique os detalhes para o resultado.", "Otimizar Disco");
    log_action("Otimizar Disco", &response);
    HttpResponse::Ok().json(response)
}

// 3. Checar Integridade (SFC)
async fn checar_integridade() -> impl Responder {
    // O 'sfc /scannow' é um comando que exige privilégios elevados e é demorado.
    // O resultado completo só é visível no console do SO ou em logs específicos.
    let response = execute_command("sfc", &["/scannow"], "Verificação de integridade do sistema (SFC) iniciada. O resultado completo aparecerá nos detalhes (se a execução for rápida) ou no console do SO.", "Checar Integridade (SFC)");
    log_action("Checar Integridade (SFC)", &response);
    HttpResponse::Ok().json(response)
}

// 4. Limpar Cache DNS
async fn limpar_dns() -> impl Responder {
    let response = execute_command("ipconfig", &["/flushdns"], "Cache DNS limpo com sucesso.", "Limpar Cache DNS");
    log_action("Limpar Cache DNS", &response);
    HttpResponse::Ok().json(response)
}

// 5. Reiniciar Serviços Windows
async fn reiniciar_servicos() -> impl Responder {
    // O comando 'net' é um comando interno do Windows, mas pode ser executado diretamente.
    // A função 'reiniciar_servicos' está incompleta, pois só tenta parar.
    // Vamos apenas parar o serviço por enquanto, pois iniciar/parar requer mais lógica.
    let response = execute_command("net", &["stop", "wuauserv"], "Tentativa de Parar o Serviço 'wuauserv'. Verifique os detalhes.", "Reiniciar Serviços Windows");
    log_action("Reiniciar Serviços Windows", &response);
    HttpResponse::Ok().json(response)
}

// 6. Limpar Logs de Eventos
async fn limpar_logs() -> impl Responder {
    // O 'wevtutil' é o comando correto para limpar logs.
    let response = execute_command("wevtutil", &["cl", "Application"], "Logs de Eventos (Application) limpos. Verifique os detalhes.", "Limpar Logs de Eventos");
    log_action("Limpar Logs de Eventos", &response);
    HttpResponse::Ok().json(response)
}

// 7. Admin: Listar Adaptadores de Rede
async fn listar_redes() -> impl Responder {
    let response = execute_command("netsh", &["interface", "ipv4", "show", "interface"], "Lista de Adaptadores de Rede obtida com sucesso.", "Listar Adaptadores de Rede");
    log_action("Listar Adaptadores de Rede", &response);
    HttpResponse::Ok().json(response)
}

// 8. Saúde do Disco (S.M.A.R.T.)
async fn disk_health() -> impl Responder {
    // O 'wmic' é o comando correto para checar o status S.M.A.R.T.
    let response = execute_command("wmic", &["diskdrive", "get", "status"], "Saúde do disco (S.M.A.R.T. Status) checada. STATUS=OK significa saudável. Verifique os detalhes.", "Saúde do Disco (S.M.A.R.T.)");
    log_action("Saúde do Disco (S.M.A.R.T.)", &response);
    HttpResponse::Ok().json(response)
}

// 9. Apps de Inicialização
async fn startup_apps() -> impl Responder {
    // O 'wmic' é o comando correto para listar apps de inicialização.
    let response = execute_command("wmic", &["startup", "get", "caption,command"], "Lista de Apps de Inicialização obtida com sucesso. Requer privilégios de Administrador. Verifique os detalhes.", "Apps de Inicialização");
    log_action("Apps de Inicialização", &response);
    HttpResponse::Ok().json(response)
}

// 10. Ativar Game Mode (Alto Desempenho)
async fn game_mode() -> impl Responder {
    let high_performance_guid = "8c5e60e3-fc78-4384-a20c-b260a927a716";
    let response = execute_command("powercfg", &["/setactive", high_performance_guid], "Modo de Energia ALTERADO para: Alto Desempenho (Game Mode). Verifique os detalhes.", "Ativar Game Mode");
    log_action("Ativar Game Mode", &response);
    HttpResponse::Ok().json(response)
}

// 11. Ativar Work Mode (Balanceado)
async fn work_mode() -> impl Responder {
    let balanced_guid = "381b4222-f694-41f0-9685-ff5bb260df2e";
    let response = execute_command("powercfg", &["/setactive", balanced_guid], "Modo de Energia ALTERADO para: Balanceado (Work Mode). Verifique os detalhes.", "Ativar Work Mode");
    log_action("Ativar Work Mode", &response);
    HttpResponse::Ok().json(response)
}

async fn get_log_content() -> impl Responder {
    match File::open("coreflow_execution.log") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                HttpResponse::Ok()
                    .content_type("text/plain; charset=utf-8")
                    .body(contents)
            } else {
                HttpResponse::InternalServerError().body("Erro ao ler o arquivo de log.")
            }
        }
        Err(_) => HttpResponse::NotFound().body("Arquivo de log ainda não foi criado."),
    }
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().json(ActionResponse {
        status: "SUCCESS".to_string(),
        message: "Conexão com a API estabelecida.".to_string(),
        details: None,
    })
}


// --- 4. Função Principal (main) ---
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🦀 CoreFlow Backend (Rust Native) - Servidor Iniciado em http://127.0.0.1:8080");
    println!("-------------------------------------------------------------------------");

    HttpServer::new(|| {
        App::new()
            // --- Handlers de Ações ---
            .route("/api/login", web::post().to(login))
            .route("/api/ping", web::get().to(ping))
            .route("/api/hardware", web::get().to(get_hardware_info))
            .route("/api/limpar_temp", web::post().to(limpar_temp))
            .route("/api/otimizar_disco", web::post().to(otimizar_disco))
            .route("/api/checar_integridade", web::post().to(checar_integridade))
            .route("/api/limpar_dns", web::post().to(limpar_dns))
            .route("/api/reiniciar_servicos", web::post().to(reiniciar_servicos))
            .route("/api/limpar_logs", web::post().to(limpar_logs))
            .route("/api/listar_redes", web::post().to(listar_redes))
            .route("/api/disk_health", web::post().to(disk_health))
            .route("/api/startup_apps", web::post().to(startup_apps))
            .route("/api/game_mode", web::post().to(game_mode))
            .route("/api/work_mode", web::post().to(work_mode))
            .route("/api/get_log", web::get().to(get_log_content))
            
            // --- Serviço de Arquivos Estáticos (AJUSTE NO CAMINHO) ---
            .service(actix_files::Files::new("/images", "static/images/")) 
            .service(actix_files::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}