trait Sortable {
    fn bub_sort(&mut self);
  }
  
  impl<T: PartialOrd + Copy> Sortable for Vec<T> {
    fn bub_sort(&mut self) {
      for i in 0 .. self.len() {
        for j in i .. self.len() {
          let tmp = self[j];
          if self[j] < self[i] {
            self[j] = self[i];
            self[i] = tmp;
          }
        }
      }
    }
  }
  
  impl<T: PartialOrd + Copy, const N: usize> Sortable for [T;N] {
    fn bub_sort(&mut self) {
      for i in 0 .. N {
        for j in i .. N {
          let tmp = self[j];
          if self[j] < self[i] {
            self[j] = self[i];
            self[i] = tmp;
          }
        }
      }
    }
  }
  
  fn main() {
    let mut arr_u32 = [5, 3, 4, 6, 1];
    let mut arr_f64 = [5.2, 3.1, 3.0, 6.22, 1.22];
    let mut vec_u32 = vec![5, 3, 4, 1, 2, 2, 46, 7, 8, 9];
    let mut vec_f64 = vec![5.0 ,3.0, 4.0, 1.0, 2.0, 2.1, 46.0, 7.0, 8.0, 9.3];
    arr_u32.bub_sort();
    println!("after sort, arr_u32 = {:?}", arr_u32);
    arr_f64.bub_sort();
    println!("after sort, arr_f64 = {:?}", arr_f64);
    vec_u32.bub_sort();
    println!("after sort, vec_u32 = {:?}", vec_u32);
    vec_f64.bub_sort();
    println!("after sort, vec_f64 = {:?}", vec_f64);
  }