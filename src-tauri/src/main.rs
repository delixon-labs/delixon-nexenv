// Previene que aparezca una ventana de consola en Windows (release)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    delixon_lib::run();
}
