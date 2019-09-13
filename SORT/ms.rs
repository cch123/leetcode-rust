fn main() {
    let mut a = vec![4,4,3,2,1];
    ms(&mut a);
    println!("{:?}",a);
}

fn ms(arr: &mut Vec<i32>) {
    if arr.len() == 0 || arr.len() == 1 {
        return
    }

    let mid = arr.len()/2;
    let (l,r ) = arr.split_at_mut(mid);
    let (mut l, mut r) = (l.to_vec(), r.to_vec());

    ms(&mut l);
    ms(&mut r);

    let (mut x,mut y, mut merged) = (0,0, vec![]);
    while x < l.len() && y < r.len() {
        if *l.get(x).unwrap() < *r.get(y).unwrap() {
            merged.push(*l.get(x).unwrap());
            x += 1;
        } else {
            merged.push(*r.get(y).unwrap());
            y+=1;
        }
    }
    while x < l.len() {
        merged.push(*l.get(x).unwrap());
        x += 1;
    }
    while y < r.len() {
        merged.push(*r.get(y).unwrap());
        y+=1;
    }

    std::mem::replace(arr, merged);
}
