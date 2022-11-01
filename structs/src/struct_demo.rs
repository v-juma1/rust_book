//定义strut
struct User {
    username: String,
    password: String,
    sign: u32,
    active: bool,
}

//定义tuple struct
struct Point(i32, i32, i32);

//当属性名与属性值对应变量名相同时，可以使用简写的方式初始化
fn build_user(username: String, password: String) -> User {
    User {
        sign: 50,
        active: false,
        //下面两个属性就是简写的方式
        username,
        password,
    }
}

//strut更新语法，基于某个struct实例来创建一个新实例
fn update_user(u1: User) -> User {
    User {
        username: String::from("jack"),

        //更新语法，表示剩下的属性都采用u1的值
        ..u1
    }
}

fn run_demo() {
    //实例化struct,可以不按照声明的顺序，但是每个属性都需要赋值
    //该struct拥有username和password变量所有权
    let mut u1 = User {
        sign: 30,
        active: true,
        username: String::from("test"),
        password: String::from("1234"),
    };

    //可变的实例每个属性都是可变的，因此可以修改实例的属性值
    u1.username = String::from("jack");

    //访问属性
    println!(
        "
        username:{}
        password:{}
        sign:{}
        active:{}
        ",
        u1.username, u1.password, u1.sign, u1.active
    );

    let username = String::from("max");
    let password = String::from("max1234");

    let u2 = build_user(username, password);
    println!(
        "
        username:{}
        password:{}
        sign:{}
        active:{}
        ",
        u2.username, u2.password, u2.sign, u2.active
    );

    let u3 = update_user(u2);
    println!(
        "
        username:{}
        password:{}
        sign:{}
        active:{}
        ",
        u3.username, u3.password, u3.sign, u3.active
    );

    //实例化一个tuple struct
    let p = Point(23, 45, 67);
    println!(
        "
        x:{}
        y:{}
        x:{}
        ",
        p.0, p.1, p.2
    );
}
