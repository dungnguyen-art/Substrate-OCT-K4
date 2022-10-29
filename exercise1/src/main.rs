fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10];
    let mut index = 1;
    let mut check: bool = true;
    for i in 0..org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            check = true;
            for j in i+1..i+sub_arr.len(){
                if org_arr[j] == sub_arr[index] {
                    index += 1;
                }
                else {
                    check = false;
                    break;
                }
            }
            if check == true {
                println!("Mang sub_arr dung la mang con cua org_arr");
                break;
            }
        }
        check = false;
    }
    if check == false {
        println!("Mang sub_arr khong phai la mang con cua org_arr");
    }
}
