/*
Para trabajar con estructuras en Rust, en primer lugar se debe definir la
estructura por nombre y especificar el tipo de dato de cada campo. Después se debe crear una
instancia de la estructura con otro nombre. Al declarar la instancia, se proporcionan
los valores específicos para los campos.
*/

//Classic structs:
struct Students {
    name: String,
    level: i8,
    remote: bool,
} //Without = nor ;

//Tupple structs:
struct Grades(char, char, char, char, f32);

//Unit structs:
struct Unit;

pub fn run() {
    // Instantiate classic struct specify fields in the same order or in random order.
    let user_1 = Students {
        name: String::from("Misha Collins"),
        level: 3,
        remote: true,
    };
    let user_2 = Students {
        name: String::from("Jared Padallecky"),
        remote: false,
        level: 2,
    };

    // Instantiate Tupple struct, pass values in the same order as types defined.
    let mark_1 = Grades('A', 'A', 'A', 'B', 4.56);
    let mark_2 = Grades('B', 'A', 'B', 'A', 7.64);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );
}
