///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////


// struct Point<T,U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }



///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng 
///////////////////////////////////////////


// Implement the generic function below.


// fn sum<T:std::ops::Add<Output = T>>(x:T,y:T)->T {
//     x + y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }




///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////



// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }


// impl A {
//     fn a(self) -> Self {
//         self.b();
//         self

//     }
//     fn b(&self) {
//         print!("b: {}", self.p.as_ref().unwrap())
//     }
// }




///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////






// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }



// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }


// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1.clone();
//     }

//     pub fn get_val2(&self) -> String {
//         return self.val2.clone();
//     }

//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1.clone(), self.val2.clone());
//     }
// }























//-------------------------------------------------------DEFINE------------------DEFINE-----------------------------//
// Dinh nghia truct:
// struct User{
//     active: bool,
//     username: String,
//     email: String, 
//     sign_in_count: f64,
// }
//==================================================== Tao instance struct va thay doi gia tri cac field ==========================================//
// fn main(){
//     let mut user1 = User{
//         email:String::from("dungtpa2@gmail.com"),
//         active:true,
//         sign_in_count:1.0,
//         username:String::from("dungnguyen"),
//     };
//     user1.email = String::from("another@gmail.com");
//     println!("{}",user1.email);
//     let mut user2 = User{
//         active:false,
//         ..user1
//     };
//     println!("{}", user2.active);
// }

//==================================================== truyen tham so vao mot ham va tao instance ==========================================//
// fn build_user(email:String, username:String) -> User{
//     let user = User{
//         email,
//         username,
//         active:false,
//         sign_in_count: 2.0,
//     };
//     user
// }

// fn main(){
//     build_user(String::from("dungtp"),String::from("ahahahah"));
// }

//==================================================== Chuong trinh don gian ve dien tich tam giac ==========================================//
// #[derive(Debug)]
// struct reatangle{
//     width:u32,
//     height:u32,
// }

// impl reatangle {
//     fn area(&self) ->u32{
//         self.width * self.height
//     }    
// }
// fn main(){
//     let reat1 = reatangle{
//         width:30,
//         height:50,
//     };
//    println!("Dien tich cua tam giac la: {}", reat1.area());
//    //dbg!(&reat1);
// }


// // fn area(reat: &reatangle)-> i32{
//     reat.width * reat.height
// }
// struct Point{
//     x:f64,
//     y:f64,
// }
// fn main(){

// }

// impl Point{
//     fn distance(&self,other:&Point)->f64{
//         let x_squared: powi(other.x - self.x,2);
//     }
// } {
    
// }