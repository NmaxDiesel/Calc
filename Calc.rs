use std::io::{self, Write};

// Fungsi untuk meminta input dari pengguna
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

// Fungsi untuk mengonversi string menjadi angka f64
fn parse_number(input: &str) -> Option<f64> {
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

// Fungsi untuk menjalankan operasi penambahan
fn add() {
    let num1 = input("Masukkan angka pertama: ");
    let num2 = input("Masukkan angka kedua: ");
    
    if let Some(num1) = parse_number(&num1) {
        if let Some(num2) = parse_number(&num2) {
            let result = num1 + num2;
            println!("Hasil: {}", result);
        } else {
            println!("Angka kedua tidak valid.");
        }
    } else {
        println!("Angka pertama tidak valid.");
    }
}

// Fungsi untuk menjalankan operasi pengurangan
fn subtract() {
    // Implementasi fungsi pengurangan di sini
}

// Fungsi untuk menjalankan operasi perkalian
fn multiply() {
    // Implementasi fungsi perkalian di sini
}

// Fungsi untuk menjalankan operasi pembagian
fn divide() {
    // Implementasi fungsi pembagian di sini
}

fn main() {
    loop {
        // Menampilkan menu operasi
        println!("Kalkulator Sederhana");
        println!("1. Tambah");
        println!("2. Kurang");
        println!("3. Kali");
        println!("4. Bagi");
        println!("5. Keluar");
        
        // Meminta input dari pengguna
        let choice = input("Pilih operasi: ");
        
        // Memproses pilihan
        match choice.as_str() {
            "1" => add(),
            "2" => subtract(),
            "3" => multiply(),
            "4" => divide(),
            "5" => {
                println!("Terima kasih! Keluar dari program.");
                break;
            }
            _ => println!("Pilihan tidak valid."),
        }
    }
}
