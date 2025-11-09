pub fn part3() {
    let names = "Harnnoris,Nexindor,Hazgnaris,Selwyris,Axalzryn,Ylarfelix,Voraxtyr,Felnketh,Braemirix,Zyrsarix,Norakthar,Talralis,Zyrtal,Fyndadir,Arvral,Arvthel,Zorgryph,Narulth,Urithdar,Nexxar,Falxeth,Zynvynar,Tarlisis,Thyrosulrix,Nylasis,Arvloris,Quirdra,Valris,Drazmir,Voraxfarin";
    let instructions = "L8,R42,L14,R33,L13,R12,L37,R10,L41,R30,L45,R36,L36,R33,L20,R9,L21,R43,L25,R12,L5,R8,L5,R9,L5,R21,L5,R46,L5,R26,L5,R39,L5,R38,L5,R30,L5,R41,L5,R11,L45,R49,L17,R27,L49,R28,L39,R5,L31,R8,L9,R14,L49,R37,L40,R22,L6,R20,L48";

    /*
    The final section also requires arranging the list of names in a closed loop. However, this time, the instructions work in a different way. The indicator
    R or L points to the name relative to the one at the top of the circle (the first on the list) that must swap places with it.
    */

    let mut names_list = names.split(",").collect::<Vec<&str>>();
    let instructions_list = instructions
        .split(",")
        .map(|x| {
            if x.starts_with("L") {
                (&x[1..]).parse::<i32>().unwrap() * -1
            } else {
                (&x[1..]).parse().unwrap()
            }
        })
        .collect::<Vec<i32>>();

    instructions_list.iter().for_each(|steps| {
        let index_to_switch = find_index(&names_list, *steps);
        names_list.swap(0, index_to_switch as usize);
    });

    println!("{:?}", names_list.first());

    fn find_index(names: &Vec<&str>, steps: i32) -> i32 {
        let rest = steps % names.len() as i32;
        if steps == 0 {
            return 0;
        }
        if steps < 0 {
            return names.len() as i32 + rest;
        }
        rest
    }
}
