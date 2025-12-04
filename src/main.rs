mod elf_database;
mod entrance_password;

fn main() {
    /*
    let password = entrance_password::get_entrance_password("safe_input");
    println!("The entrance password is '{}'", password);
    */

    let invalid_id_sum = elf_database::count_invalid_ids("elf_database_ranges");
    println!("The invalid ID sum is {}", invalid_id_sum);
}
