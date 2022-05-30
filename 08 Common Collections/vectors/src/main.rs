fn main() {
    {
        let v1: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        println!("Vector after immutable loop {:?}", v);
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        println!("Vector after mutable loop {:?}", v);
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
