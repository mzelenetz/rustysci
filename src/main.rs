fn main() { 
    let v1:Vec<f32>=vec![1.2,2.,3.,4.];
    let v2:Vec<f32>=vec![2.,3.,4.,6.];
    let v3:Vec<f32>=vec![2.,3.,4.,6.];
    let v4:Vec<f32>=vec![1.,30.,-4.,-60.];

    let v5:Vec<f32>=vec![1.,2.];
    let v6:Vec<f32>=vec![3.,4.];
    let v7:Vec<f32>=vec![5.,6.];

    println!("{:?}", add(&v1,&v2));
    println!("{:?}", subtract(&v1,&v2));
    println!("{:?}", vector_sum(&vec![&v1,&v2,&v3, &v4]));
    println!("{:?}", scalar_multiply(&2., &v1));
    println!("{:?}", vector_mean(&vec![&v5, &v6, &v7]))
}

fn add(v1:&Vec<f32>, v2:&Vec<f32>)->Vec<f32>{
    assert_eq!(v1.len(), v2.len());
    let mut res=Vec::new();
    
    for (i,v) in v1.iter().enumerate() {
        res.push(v + v2[i]);
    }
    return res
}

fn subtract(v1:&Vec<f32>, v2:&Vec<f32>)->Vec<f32>{
    assert_eq!(v1.len(), v2.len());
    let mut res=Vec::new();
    
    for (i,v) in v1.iter().enumerate() {
        res.push(v - v2[i]);
    }
    return res
}

fn scalar_multiply(c:&f32, v:&Vec<f32>)->Vec<f32>{
    let mut res = Vec::new();
    for i in v.iter(){
        res.push(i*c);
    }
    res
    
}

fn vector_sum(vectors:&Vec<&Vec<f32>>)->Vec<f32>{
    // TODO: need to check we got vectors

    // Check all are the same size
    let all_equal = vectors.iter().all(|ref v| v.len() == vectors[0].len());
    assert_eq!(all_equal, true);

    let mut res = Vec::new();    
    let mut summer:f32=0.0;
    for i in 0..vectors[0].len(){
        summer = 0.0;
        for vector in vectors.iter(){
            summer += vector[i];
        }
        res.push(summer);
    }
    res
}

fn vector_mean(vectors:&Vec<&Vec<f32>>)->Vec<f32>{
    // Check all are the same size
    let all_equal = vectors.iter().all(|ref v| v.len() == vectors[0].len());
    assert_eq!(all_equal, true);

    let n:f32 = vectors[0].len() as f32;
    scalar_multiply(&(1./n), &vector_sum(&vectors))
}