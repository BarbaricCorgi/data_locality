use std::time::Instant;
use std::mem::size_of_val;
use std::thread;

#[derive(Copy, Clone)]
struct Cell {
    value: f64,
    formula: i64,
    dependants: i64
}
fn main() {
    const MAX_MEMORY:usize = 4000 * 1024 * 1024; //4000MB stack
    
    let builder = thread::Builder::new()
    .name("reductor".into())
    .stack_size(MAX_MEMORY);

    let handler = builder.spawn(|| {

        const CELLS_QTY:usize  = 10000000;
        let mut cells = [Cell{ value: 1.0, formula: 1, dependants: 2 }; CELLS_QTY ];
        
        println!("Sizeof Struct: {}", size_of_val(&cells[0]));
    
        let now = Instant::now();
        for n in 0..(CELLS_QTY - 1)/ 3 {
            cells[(n * 3) + 2].value = cells[(n * 3)].value + cells[(n * 3) + 1].value; 
        }
        println!("Struct version {}", now.elapsed().as_millis());
    
         
        let mut values = [1.0; CELLS_QTY ];
        //let formulas = [1; CELLS_QTY ];
        //let dependants = [1; CELLS_QTY ];    
    
        let now2= Instant::now();
        for n in 0..(CELLS_QTY - 1)/ 3 {
            values[(n * 3) + 2] = values[(n * 3)] + values[(n * 3) + 1]; 
        }
        
        println!("Columnar version {}", now2.elapsed().as_millis());
    }).unwrap();

    handler.join().unwrap();
}
