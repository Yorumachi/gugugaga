fn main() {
    九九乘法表();
    pascal_triangle(8);
    calc_sum(100);
    print!("rust太他妈的快了")
}


fn 九九乘法表(){
    for i in 1..=9 {
        for j in 1..=i {
            print!("{}×{}={:<2}  ", j, i, i * j);
        }
        println!();
    }
}

fn pascal_triangle(n: usize) {
    let mut current_row = vec![1];
    for i in 0..n {
        print!("{:>width$}", "", width = (n - i - 1) * 2);
        for &num in &current_row {
            print!("{:>4}", num);
        }
        println!();
        if i < n - 1 {
            let mut next_row = vec![1];
            for j in 0..current_row.len() - 1 {
                next_row.push(current_row[j] + current_row[j + 1]);
            }
            next_row.push(1);
            current_row = next_row;
        }
    }
}

fn calc_sum(n:i32){
    let mut sum:i32 = 0;
    for i in 0..n+1{
        sum += i
    }
    print!("从0到{}的和为{}",n,sum)
}