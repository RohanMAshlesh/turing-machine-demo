use std::io;

use std::collections::HashMap;

// Instruction format:
// [ current state : input : output : left|right : next state ]
// type Instr = [u8; 5];

// Helper function
fn copy_into_array<A, T>(slice: &[T]) -> A
where
  A: Sized + Default + AsMut<[T]>,
  T: Copy,
{
  let mut a = Default::default();
  <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
  a
}

// Map for storing the transitions- each transition is identifiable by the current state and input.
//                    prev, input   move, output, next
type Transition = HashMap<[u8; 2], [u8; 3]>;

// The universal turing machine
#[derive(Debug)]
struct Turing {
  transitions: Transition,
  tape: Vec<u8>,
  pos: usize,
  curr: u8,
}

impl Turing {
  // Constructor
  pub fn new() -> Turing {
    Turing {
      transitions: Transition::new(),
      tape: vec![b'B'; 64],
      pos: 32,
      curr: 0,
    }
  }

  // The input consists of a comma separated list of transitions. The states are inferred from the transitions.
  // Eg - 001R0,010R0,0BBL1,101L2,200L2,211L2,2BBR5,110L3,310L3,301L2
  // These are the transitions for a turing machine to get the 2's complement of a binary number.
  // Instruction format-
  // [ current state : input : output : 'L' or 'R' : next state ]
  // Current and next states are unsigned integers; input and output are characters.
  pub fn set_input(&mut self, input: &str) -> &mut Turing {
    let mut x: [u8; 5];
    for i in input.split(',') {
      x = copy_into_array(i.as_bytes());

      // We convert current and next into numbers.
      x[0] -= b'0';
      x[4] -= b'0';
      self
        .transitions
        .insert(copy_into_array(&x[..2]), copy_into_array(&x[2..]));
    }
    self
  }

  // pub fn get_transitions(&self) -> String {
  //   format!("{:?}", self.transitions)
  // }

  // THis function runs for each transition of the machine.
  // It returns true if a transition was successfully made,
  //        and false if there are no possible transitions.
  //
  pub fn step(&mut self) -> bool {
    // Retrieve the transition corresponding to the current state and input.
    if let Some(tr) = self.transitions.get(&[self.curr, self.tape[self.pos]]) {
      println!(
        "Transition [ state1: {}, input: {}, output: {}, move: {}, state2: {} ]",
        self.curr, self.tape[self.pos] as char, tr[0] as char, tr[1] as char, tr[2]
      );

      // Update with new value.
      self.tape[self.pos] = tr[0];

      match tr[1] as char {
        'L' => self.pos -= 1,
        'R' => self.pos += 1,
        // If the move isn't left or right.
        _ => panic!("Invalid move at state {}", self.curr),
      }

      self.curr = tr[2];
    } else {
      // No transition from current state for the given input.
      println!("No transitions possible!");
      return false;
    }

    println!(
      "{} : Pos : {}",
      String::from_utf8(self.tape[30..self.pos + 16].to_vec()).unwrap(),
      self.pos - 31
    );
    println!("{}^", String::from_utf8(vec![b'-'; self.pos - 30]).unwrap());

    true
  }

  pub fn set_tape(&mut self, tape: &str) -> &mut Turing {
    for (pos, &i) in tape.as_bytes().iter().enumerate() {
      self.tape.insert(pos + 32, i);
    }

    println!("{}", String::from_utf8(self.tape.clone()).unwrap());

    self
  }

  fn get_tape(&self) -> String {
    String::from_utf8(self.tape.clone()).unwrap()
  }
}

fn main() {
  let mut t: Turing = Turing::new();
  let mut input = String::new();
  let mut tape = String::new();
  let stdin = io::stdin();

  println!("Enter input for tape:");
  match stdin.read_line(&mut tape) {
    Ok(_) => {
      println!("Enter program for turing machine");
      match stdin.read_line(&mut input) {
        Ok(_) => {}
        _ => return,
      }
    }
    _ => return,
  }

  t.set_input(&input.trim()).set_tape(&tape.trim());

  // Keep stepping the machine until we can't make any more transitions.
  loop {
    if !t.step() {
      break;
    }
  }

  println!("\nInput: {}Final result: {}", tape, t.get_tape());
}
