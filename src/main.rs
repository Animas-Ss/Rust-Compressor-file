extern crate flate2;

//TODO: las librerias que vamosa  usar tando del crate flate2 como las estandar
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;// son los argumentos de entrada 
use std::fs::File;//para usar los archivos
use std::io::BufReader;//poder leer buffer
use std::io::copy;//copar
use std::time::Instant;// timepo



fn main() {
    if args().len() != 3 {
        eprintln!("Use: `Archivo Origen` `Nombre del archivo de Salida`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    println!("tamaño de entrada: {:?}", input.get_ref().metadata().unwrap().len());
    println!("tamaño de salida: {:?}", output.metadata().unwrap().len());
    println!("Timpo de demora: {:?}", start.elapsed());

}
