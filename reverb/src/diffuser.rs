// use super::allpass_filter::AllpassFilter;

// struct DiffuserElement {
//   allpass_filter: AllpassFilter,
//   time: f32,
// }
// pub struct Diffuser {
//   diffuser_elements: Vec<DiffuserElement>,
// }

// impl Diffuser {
//   pub fn new(sample_rate: f64, times: [f32; 4]) -> Self {
//     Self {
//       diffuser_elements: times
//         .iter()
//         .map(|time| DiffuserElement {
//           allpass_filter: AllpassFilter::new(sample_rate),
//           time: *time,
//         })
//         .collect(),
//     }
//   }

//   pub fn run(&mut self, input: f32, diffuse: f32) -> f32 {
//     self
//       .diffuser_elements
//       .iter_mut()
//       .fold(input, |accum, item| -> f32 {
//         item.allpass_filter.run(accum, item.time, diffuse)
//       })
//   }
// }

use super::allpass_filter::AllpassFilter;

pub struct Diffuser {
  allpass_filter: Vec<AllpassFilter>,
  times: [f32; 4],
}

impl Diffuser {
  pub fn new(sample_rate: f64, times: [f32; 4]) -> Self {
    Self {
      allpass_filter: vec![AllpassFilter::new(sample_rate); 4],
      times,
    }
  }

  pub fn run(&mut self, input: f32, diffuse: f32) -> f32 {
    let Diffuser {
      allpass_filter,
      times,
    } = self;
    let allpass1 = allpass_filter[0].run(input, times[0], diffuse);
    let allpass2 = allpass_filter[1].run(allpass1, times[1], diffuse);
    let allpass3 = allpass_filter[2].run(allpass2, times[2], diffuse);
    allpass_filter[3].run(allpass3, times[3], diffuse)
  }
}
