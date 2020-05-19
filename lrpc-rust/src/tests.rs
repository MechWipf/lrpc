use crate::*;

#[test]
fn test_val() {
    let ref mut q = ByteQue::new();

    let v: usize = 65536;
    v.store(q);
    assert_eq!(v, usize::restore(q));

    for _ in 0..25 {
        0x80u8.store(q);
    }
    assert_eq!(usize::restore(q), 0);

    let ref mut q = ByteQue::new();

    let v: i8 = -1;
    v.store(q);
    assert_eq!(v, i8::restore(q));

    let v: i16 = -1;
    v.store(q);
    assert_eq!(v, i16::restore(q));

    let v: i32 = -1;
    v.store(q);
    assert_eq!(v, i32::restore(q));

    let v: i64 = -1;
    v.store(q);
    assert_eq!(v, i64::restore(q));

    let v: i128 = -1;
    v.store(q);
    assert_eq!(v, i128::restore(q));

    let v: f32 = -3.141592654;
    v.store(q);
    assert_eq!(v, f32::restore(q));

    let v: f64 = -3.141592654;
    v.store(q);
    assert_eq!(v, f64::restore(q));

    let v: bool = true;
    v.store(q);
    assert_eq!(v, bool::restore(q));

    let v: u8 = 255;
    v.store(q);
    assert_eq!(v, u8::restore(q));

    let v: u16 = 65535;
    v.store(q);
    assert_eq!(v, u16::restore(q));

    let v: u32 = 4294967295;
    v.store(q);
    assert_eq!(v, u32::restore(q));

    let v: u64 = 4294967295;
    v.store(q);
    assert_eq!(v, u64::restore(q));

    let v: u128 = 4294967295;
    v.store(q);
    assert_eq!(v, u128::restore(q));

    let v = '😄';
    v.store(q);
    assert_eq!(v, char::restore(q));

    let v = "welcome to 😄".to_string();
    v.store(q);
    assert_eq!(v, String::restore(q));

    let v = ();
    v.store(q);
    assert_eq!(v, <()>::restore(q));

    let v = Some("Hello World!".to_string());
    v.store(q);
    assert_eq!(v, Option::<String>::restore(q));

    let v: Option<String> = None;
    v.store(q);
    assert_eq!(v, Option::<String>::restore(q));

    let v = Box::new(996);
    v.store(q);
    assert_eq!(v, Box::<i32>::restore(q));

    let v = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
    ];
    v.store(q);
    assert_eq!(v, Vec::<String>::restore(q));

    let mut v = std::collections::HashMap::<String, i32>::new();
    v.insert("one".to_string(), 1);
    v.insert("two".to_string(), 2);
    v.insert("three".to_string(), 3);
    v.insert("four".to_string(), 4);
    v.insert("five".to_string(), 5);
    v.store(q);
    assert_eq!(v, std::collections::HashMap::<String, i32>::restore(q));

    let v = (1u8, "two".to_string(), 3i16, 4u32, "five".to_string());
    v.store(q);
    assert_eq!(v, Store::restore(q));
}

#[test]
fn test_fun() {
    fn adder(q: &mut ByteQue) -> ByteQue {
        let x = i32::restore(q);
        let y = i32::restore(q);
        let z = x + y;
        let mut r = ByteQue::new();
        if z == 0 {
            Result::<i32>::Err("parameter error".to_string()).store(&mut r);
        } else {
            Ok(z).store(&mut r);
        }
        r
    }
    let mut fun = Fun::new();
    fun.regist("adder", adder);
    let mut r = fun!("adder", 1, 1);
    assert_eq!(Result::<i32>::restore(&mut fun.invoke(&mut r)), Ok(2));
}

#[test]
fn test_derive_macros() {
    let mut q = ByteQue::new();

    #[derive(CommonStore)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let v1 = Rectangle {
        width: 800,
        height: 600,
    };
    v1.store(&mut q);
    let v2 = Rectangle::restore(&mut q);
    assert!(v1.width == v2.width && v1.height == v2.height);

    #[derive(CommonStore)]
    struct Expiration<T: Store>(Option<T>);
    let v1: Expiration<i32> = Expiration(Some(18));
    v1.store(&mut q);
    let v2: Expiration<i32> = Store::restore(&mut q);
    assert_eq!(v1.0, v2.0);

    #[derive(CommonStore)]
    enum State {
        Prefix = 0,
        StartDir = 1,
        Body = 2,
        Done = 3,
    }
    let v1 = State::Body;
    v1.store(&mut q);
    let v2 = State::restore(&mut q);
    assert!(matches!(v2, State::Body));

    #[derive(CommonStore)]
    pub(crate) enum ReadStrategy<T: Store> {
        First,
        Second,
        Adaptive {
            decrease_now: bool,
            next: usize,
            max: usize,
        },
        Exact(usize, T),
        Tail,
    }
    let v1: ReadStrategy<String> = ReadStrategy::Adaptive {
        decrease_now: true,
        next: 1,
        max: 2,
    };
    v1.store(&mut q);
    let v2: ReadStrategy<String> = ReadStrategy::restore(&mut q);
    let v = match v2 {
        ReadStrategy::Adaptive {
            decrease_now,
            next,
            max,
        } => (decrease_now, next, max),
        _ => (false, 0, 0),
    };
    assert!(v.0 && v.1 == 1 && v.2 == 2);
}

#[test]
fn test_attribute_macro() {
    let mut fun = Fun::new();

    #[fmt_function]
    fn first_word(s: String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    fun.regist("first_word", first_word);
    let r: Result<usize> =
        Store::restore(&mut fun.invoke(&mut fun!("first_word", String::from("hello world"))));
    assert_eq!(r, Ok(5));

    #[fmt_function]
    fn read_username_from_file() -> std::io::Result<String> {
        let mut s = String::new();
        std::fs::File::open("hello.txt")?;
        Ok(s)
    }
    fun.regist("read_username_from_file", read_username_from_file);
    let r: Result<String> = Store::restore(&mut fun.invoke(&mut fun!("read_username_from_file")));
    assert!(r.is_err());

    struct Student {
        name: String,
        age: i32,
    }
    impl Student {
        fn new() -> Self {
            Student {
                name: String::new(),
                age: 0,
            }
        }
        #[fmt_function]
        fn set_name_age(&mut self, name: String, age: i32) {
            self.name = name;
            self.age = age;
        }
        #[fmt_function]
        fn next_year(&self) -> i32 {
            self.age + 1
        }
    }
    let mut student = Student::new();

    let mut fun = fun!("set_name_age", "chenchen".to_string(), 18);
    if String::restore(&mut fun) == "set_name_age" {
        let r: Result<()> = Store::restore(&mut student.set_name_age(&mut fun));
        assert_eq!(r, Ok(()));
    }

    let mut fun = fun!("next_year");
    if String::restore(&mut fun) == "next_year" {
        let r: Result<i32> = Store::restore(&mut student.next_year(&mut fun));
        assert_eq!(r, Ok(19));
    }
}

#[test]
fn test_buf() {
    let mut q = ByteQue::new();
    128u8.store(&mut q);
    let mut r = RecvBuf::new();
    r.append(Vec::<u8>::from(q).as_slice());
    assert_eq!(r.size(), None);

    let mut q = ByteQue::new();
    127u8.store(&mut q);
    let mut r = RecvBuf::new();
    r.append(Vec::<u8>::from(q).as_slice());
    assert_eq!(r.size(), Some(127));

    let mut q = ByteQue::new();
    2u8.store(&mut q);
    1u8.store(&mut q);
    2u8.store(&mut q);
    3u8.store(&mut q);
    4u8.store(&mut q);
    5u8.store(&mut q);
    let mut r = RecvBuf::new();
    r.append(Vec::<u8>::from(q).as_slice());
    assert_eq!(Vec::<u8>::from(ByteQue::from(r)), vec![1, 2]);

    let s = "hello world!".to_string();
    let mut q = ByteQue::new();
    s.store(&mut q);
    let v = send_data(q);
    assert!(v[0] == 13 && v[1] == 12);
    let mut r = RecvBuf::new();
    r.append(v.as_slice());
    let mut q = ByteQue::from(r);
    assert_eq!(String::restore(&mut q), s);
}

#[test]
fn test_tcp() {
    #[derive(CommonStore)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    #[fmt_function]
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[fmt_function]
    fn bublle1(arr: Vec<i32>) -> Vec<i32> {
        bubble_sort(arr)
    }
    fn bublle2(arr: Vec<i32>) -> Vec<i32> {
        bubble_sort(arr)
    }
    fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
        for i in 0..arr.len() - 1 {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j + 1] {
                    let temp = arr[j + 1];
                    arr[j + 1] = arr[j];
                    arr[j] = temp;
                }
            }
        }
        arr
    }

    let mut fun = Fun::new();
    fun.regist("value_in_cents", value_in_cents);
    fun.regist("bublle1", bublle1);

    std::thread::spawn(move || {
        service(fun, "0.0.0.0:9009");
    });
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut con = Connection::new("127.0.0.1:9009");

    let rst: Result<u8> = con.invoke(fun!("value_in_cents", Coin::Quarter));
    assert_eq!(rst, Ok(25));

    let mut arr: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..1000 {
        arr.push(1000 - i);
    }
    let t1 = std::time::Instant::now();
    let r1: Result<Vec<i32>> = con.invoke(fun!("bublle1", arr));
    let t2 = std::time::Instant::now();
    dbg!(t2 - t1);
    let r2 = bublle2(arr);
    let t3 = std::time::Instant::now();
    dbg!(t3 - t2);
    assert_eq!(r1.unwrap(), r2);
}
