use std::fs;
use std::path::{Path, PathBuf}; 
use std::io;
use chrono::Utc;

pub fn backup_path<P: AsRef<Path>>(source: P, backups_base: &str) -> Result<String, String> {
    let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let source = source.as_ref();
    
    // 1. Verificação de existência
    if !source.exists() {
        return Err(format!("source does not exist: {}", source.display()));
    }
    
    // 2. Determinação do nome do diretório de destino
    let filename = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "backup".to_string());
        
    // O diretório de destino do backup terá o nome original + timestamp
    let dest_dir = Path::new(backups_base).join(format!("{}_{}", filename, now));
    
    // 3. Criação do diretório base
    fs::create_dir_all(&dest_dir).map_err(|e| format!("mkdir err: {:?}", e))?;
    
    // 4. Copia
    if source.is_file() {
        // Se a fonte for um arquivo, copiamos o arquivo diretamente para dentro do novo diretório criado (dest_dir).
        let dest_file = dest_dir.join(filename);
        fs::copy(source, &dest_file)
             // Mapeamos o erro io::Error para String, necessário devido à assinatura da função (Result<String, String>).
            .map_err(|e| format!("copy err: {:?}", e))?; 
    } else {
        // Se for um diretório, chamamos a cópia recursiva, a partir da raiz do diretório.
        copy_recursively(source, &dest_dir)
             // Mapeamos o erro io::Error para String.
            .map_err(|e| format!("copy err: {:?}", e))?;
    }
    
    Ok(dest_dir.to_string_lossy().to_string())
}

// Lógica de cópia recursiva corrigida.
fn copy_recursively(src: &Path, dst: &Path) -> io::Result<()> {
    // Se a fonte é um arquivo.
    if src.is_file() {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        // ERRO 2 CORRIGIDO: Agora a cópia está completa e retorna o Result.
        fs::copy(src, dst)?; 
        // ERRO 1 CORRIGIDO: O fluxo da função deve terminar aqui se for um arquivo.
        return Ok(()); 
    }
    
    // Se a fonte é um diretório, criamos o diretório de destino.
    fs::create_dir_all(dst)?;
    
    // Iteramos sobre as entradas do diretório de origem.
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let p = entry.path();
        let file_name = entry.file_name();
        
        // O destino é o diretório de destino (dst) unido ao nome do item.
        let dest = dst.join(file_name); 
        
        if p.is_dir() {
            // Chamada recursiva para pastas
            copy_recursively(&p, &dest)?;
        } else {
            // Cópia direta para arquivos