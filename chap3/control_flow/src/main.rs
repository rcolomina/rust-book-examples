
fn multiple_loops() {

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn condition_while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn condition_while_2(){

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    
    let max_index = 5;
    while index < max_index {
        println!("the value is: {}", a[index]);
        
        index += 1;
    }
}

fn for_loop_1(){
    let a = [1,2,3,4,56];
    for elem in a {
        println!("vale is {elem}");
    }

}


fn for_loop_reverse(){

    for number in (1..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {

    multiple_loops();
    condition_while();
    condition_while_2();
    for_loop_1();
    for_loop_reverse();

}
