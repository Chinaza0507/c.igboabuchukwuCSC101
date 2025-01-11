use std::fs::File;
use std::io::Write;
fn main() {
    let mut file = File::create("Merged data.txt").expect("failed to create");
    let comissioners = ["Aigbogun Alamba Daudu", "Muritala Afeez Bendu", "Okorocha Calistus Ogbonna", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let  geopolitical_zones = ["South West", "North East", "South South", "South West", "South East"];

    file.write_all("S/N,Name of Commisioner, Ministry, Geopolitical Zone\n".as_bytes()).expect("Failed to write to file");
    for i in 0..5 {
       let row = format!(
"{}, {}, {}, {}\n",
i +1,
comissioners[i],
ministries[i],
geopolitical_zones[i]

        );
       file.write_all(row.as_bytes()).expect("failed to write row");
    }
    println!("data saved to merged data.txt");
}
