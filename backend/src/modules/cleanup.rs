use std::env;
use std::fs;
use std::path::PathBuf;

// Importação crucial: precisamos de 'Result' para que o `unwrap_or_else` funcione
use std::io::Result; 

// Importação da crate de serialização, necessária se você usar Value.
use serde_json; 

use super::backup;

// Função para simular o que será deletado
pub fn clean_temp_dryrun() -> serde_json::Value {
    // Tenta obter a variável de ambiente TEMP; se falhar, usa o caminho padrão do Windows
    let temp = env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    
    // Retorna um objeto JSON simples
    serde_json::json!({"target": temp, "action": "delete_files_recursive", "dry_run": true})
}

// Função para executar a limpeza (com backup primeiro)
pub fn clean_temp_execute(backup_folder: &str) -> serde_json::Value {
    // Obtém o caminho da pasta TEMP
    let temp = env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    
    // Tenta fazer o backup usando a função do módulo 'backup'
    let backup_result = backup::backup_path(&temp, backup_folder);
    
    // Constrói a resposta JSON, tratando o resultado do backup:
    // Se for Ok, pega a string do caminho do backup; se for Err, formata o erro.
    serde_json::json!({
        "target": temp, 
        "backup": backup_result.unwrap_or_else(|e| format!("err: {:?}", e)), 
        "deleted": false, 
        "note": "Deletion is protected; enable explicit confirm to delete."
    })
}