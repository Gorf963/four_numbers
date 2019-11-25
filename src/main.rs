use std::time::Instant;

#[derive(Clone)]
struct Storage
{
    number: f32,
    s: String,
    size: i8,
}
impl Storage {
    pub fn new(number:f32, s: String, size: i8)  -> Storage {
        Storage {
            number: number,
            s: s,
            size: size,
        }
    }
    pub fn to_string(&self) -> String {
        let mut _output = String::new();
        _output = format!("Number: {}, String: {}, Size: {}", self.number, self.s, self.size);
        return _output;
    }
}

fn main() {
    let mut m: Vec<Storage> = Vec::new();

    let start = Instant::now();

    //initalize array
    for i in {1..13} {
        m.push(Storage::new(i as f32, i.to_string(),1));
    }
    //Cards 2-->4
    for j in {1..4} {
        //Add next card
        for index in {1..13} {
            add(j, index, &mut m);
            sub(j, index, &mut m);
            mult(j, index, &mut m);
            div(j, index, &mut m);
        }
    }
    let duration = start.elapsed();
    let start = Instant::now();
    for p in m.iter().filter(|n|n.size==4&&n.number<24.1&&n.number>23.9) {
        println!("{}",p.to_string());
    }
    println!("Calc duration {:?}", duration);
    println!("Output took {:?}", start.elapsed());
    println!("Count {}",m.iter().filter(|n|n.size==2&&n.number<24.1&&n.number>23.9).count()  );
}


fn add(level:i8, index: i8, m: &mut Vec<Storage>){
    let temp_m: Vec<Storage> = m.iter().filter(|n|n.size==level).cloned().collect();   
    for n in temp_m {
        m.push(
            Storage::new(
                 n.number+index as f32, 
                format!("({})+{}", n.s, index),
                n.size + 1
            )
        )
    }
}
fn sub(level:i8, index: i8, m: &mut Vec<Storage>){
    let temp_m: Vec<Storage> = m.iter().filter(|n|n.size==level).cloned().collect();   
    for n in temp_m {
        m.push(
            Storage::new(
                 n.number-index as f32, 
                format!("({})-{}", n.s, index),
                n.size + 1
            )
        );
        m.push(
            Storage::new(
                index as f32 - n.number, 
                format!("{}-({})", index, n.s),
                n.size + 1
            )
        );
    }
}
fn mult(level:i8, index: i8, m: &mut Vec<Storage>){
    let temp_m: Vec<Storage> = m.iter().filter(|n|n.size==level).cloned().collect();   
    for n in temp_m {
        m.push(
            Storage::new(
                n.number * index as f32, 
                format!("({})*{}", n.s, index),
                n.size + 1
            )
        )
    }
}
fn div(level:i8, index: i8, m: &mut Vec<Storage>){
    let temp_m: Vec<Storage> = m.iter().filter(|n|n.size==level).cloned().collect();   
    for n in temp_m {
        m.push(
            Storage::new(
                 n.number/index as f32, 
                format!("({})/{}", n.s, index),
                n.size + 1
            )
        );
        m.push(
            Storage::new(
                index as f32 / n.number, 
                format!("{}/({})", index, n.s),
                n.size + 1
            )
        );
    }
}