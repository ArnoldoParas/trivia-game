use std::env;
use std::path::Path;
use fs_extra::dir::{self, CopyOptions};

fn main() {
    // Obtener directorios necesarios
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("No se pudo obtener CARGO_MANIFEST_DIR");
    let profile = env::var("PROFILE").expect("No se pudo obtener PROFILE");
    
    // Configurar rutas
    let assets_source = Path::new(&manifest_dir).join("assets");
    let target_dir = Path::new(&manifest_dir).join("target").join(&profile);
    
    println!("cargo:warning=Copiando assets de: {}", assets_source.display());
    println!("cargo:warning=Hacia: {}", target_dir.display());
    
    // Crear directorio de destino si no existe
    std::fs::create_dir_all(&target_dir).expect("No se pudo crear el directorio de destino");
    
    // Configurar opciones de copiado
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    
    // Copiar los archivos
    match dir::copy(&assets_source, &target_dir, &copy_options) {
        Ok(_) => println!("cargo:warning=Assets copiados exitosamente"),
        Err(e) => panic!("Error al copiar assets: {}", e)
    }
    
    // Notificar a Cargo que debe reconstruir si los assets cambian
    println!("cargo:rerun-if-changed=assets/");
}