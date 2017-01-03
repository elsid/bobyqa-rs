use std::os::raw::c_void;

type Function = extern fn(data: *const c_void, n: i64, x: *const f64) -> f64;

#[repr(C)]
struct Closure {
    data: *const c_void,
    function: Function,
}

impl Closure {
    pub fn new<F>(function: &F) -> Closure where F: Fn(&[f64]) -> f64 {
        extern fn wrap<F>(closure: *const c_void, n: i64, x: *const f64) -> f64
                where F: Fn(&[f64]) -> f64 {
            use std::slice::from_raw_parts;
            let closure = closure as *const F;
            unsafe { (*closure)(from_raw_parts(x, n as usize)) }
        }
        Closure {data: &*function as *const _ as *const c_void, function: wrap::<F>}
    }
}

type FunctionMut = extern fn(data: *mut c_void, n: i64, x: *const f64) -> f64;

#[repr(C)]
struct ClosureMut {
    data: *mut c_void,
    function: FunctionMut,
}

impl ClosureMut {
    pub fn new<F>(function: &mut F) -> ClosureMut where F: FnMut(&[f64]) -> f64 {
        extern fn wrap<F>(closure: *mut c_void, n: i64, x: *const f64) -> f64
                where F: FnMut(&[f64]) -> f64 {
            use std::slice::from_raw_parts;
            let closure = closure as *mut F;
            unsafe { (*closure)(from_raw_parts(x, n as usize)) }
        }
        ClosureMut {data: &mut *function as *mut _ as *mut c_void, function: wrap::<F>}
    }
}

extern "C" {
    fn bobyqa_closure(function: *mut ClosureMut, n: i64, npt: i64, x: *mut f64,
        xl: *const f64, xu: *const f64, rhobeg: f64, rhoend: f64, maxfun: i64, w: *mut f64) -> f64;

    fn bobyqa_closure_const(function: *const Closure, n: i64, npt: i64, x: *mut f64,
        xl: *const f64, xu: *const f64, rhobeg: f64, rhoend: f64, maxfun: i64, w: *mut f64) -> f64;
}

#[derive(Clone)]
pub struct Bobyqa <'a> {
    variables_count: usize,
    number_of_interpolation_conditions: usize,
    initial_trust_region_radius: f64,
    final_trust_region_radius: f64,
    lower_bound: Option<&'a [f64]>,
    upper_bound: Option<&'a [f64]>,
    max_function_calls_count: usize,
}

impl <'a> Bobyqa <'a> {
    pub fn new() -> Bobyqa <'a> {
        const VARIABLES_COUNT: usize = 2;
        const NUMBER_OF_INTERPOLATION_CONDITIONS: usize = VARIABLES_COUNT + 2;
        static LOWER_BOUND: [f64; VARIABLES_COUNT] = [0.0, 0.0];
        static UPPER_BOUND: [f64; VARIABLES_COUNT] = [1.0, 1.0];
        Bobyqa {
            variables_count: VARIABLES_COUNT,
            number_of_interpolation_conditions: NUMBER_OF_INTERPOLATION_CONDITIONS,
            lower_bound: Some(&LOWER_BOUND),
            upper_bound: Some(&UPPER_BOUND),
            initial_trust_region_radius: 1e-6,
            final_trust_region_radius: 1e6,
            max_function_calls_count: 1000,
        }
    }

    pub fn variables_count(&mut self, value: usize) -> &mut Self {
        assert!(value >= 2);
        self.variables_count = value;
        if self.lower_bound.is_some() && self.lower_bound.unwrap().len() < self.variables_count {
            self.lower_bound = None;
        }
        if self.upper_bound.is_some() && self.upper_bound.unwrap().len() < self.variables_count {
            self.upper_bound = None;
        }
        self
    }

    pub fn number_of_interpolation_conditions(&mut self, value: usize) -> &mut Self {
        assert!(value >= 4);
        self.number_of_interpolation_conditions = value;
        self
    }

    pub fn lower_bound(&mut self, value: &'a [f64]) -> &mut Self {
        assert!(value.len() >= self.variables_count);
        self.lower_bound = Some(value);
        self
    }

    pub fn upper_bound(&mut self, value: &'a [f64]) -> &mut Self {
        assert!(value.len() >= self.variables_count);
        self.upper_bound = Some(value);
        self
    }

    pub fn initial_trust_region_radius(&mut self, value: f64) -> &mut Self {
        assert!(value <= self.final_trust_region_radius);
        self.initial_trust_region_radius = value;
        self
    }

    pub fn final_trust_region_radius(&mut self, value: f64) -> &mut Self {
        assert!(value >= self.initial_trust_region_radius);
        self.final_trust_region_radius = value;
        self
    }

    pub fn max_function_calls_count(&mut self, value: usize) -> &mut Self {
        self.max_function_calls_count = value;
        self
    }

    pub fn perform<F>(&self, values: &mut [f64], function: &F) -> f64
            where F: Fn(&[f64]) -> f64 {
        self.check(values);
        let closure = Closure::new(function);
        let mut working_space = self.working_space();
        unsafe {
            bobyqa_closure_const(
                &closure as *const _,
                self.variables_count as i64,
                self.number_of_interpolation_conditions as i64,
                values.as_mut_ptr(),
                self.lower_bound.unwrap().as_ptr(),
                self.upper_bound.unwrap().as_ptr(),
                self.initial_trust_region_radius,
                self.final_trust_region_radius,
                self.max_function_calls_count as i64,
                working_space.as_mut_ptr(),
            )
        }
    }

    pub fn perform_mut<F>(&self, values: &mut [f64], function: &mut F) -> f64
            where F: FnMut(&[f64]) -> f64 {
        self.check(values);
        let mut closure = ClosureMut::new(function);
        let mut working_space = self.working_space();
        unsafe {
            bobyqa_closure(
                &mut closure as *mut _,
                self.variables_count as i64,
                self.number_of_interpolation_conditions as i64,
                values.as_mut_ptr(),
                self.lower_bound.unwrap().as_ptr(),
                self.upper_bound.unwrap().as_ptr(),
                self.initial_trust_region_radius,
                self.final_trust_region_radius,
                self.max_function_calls_count as i64,
                working_space.as_mut_ptr(),
            )
        }
    }

    fn check(&self, values: &[f64]) {
        assert!(values.len() >= self.variables_count);
        assert!(self.number_of_interpolation_conditions >= self.variables_count + 2);
        assert!(self.number_of_interpolation_conditions <=
            (self.variables_count + 1)*(self.variables_count + 2)/2);
        assert!(self.lower_bound.is_some());
        assert!(self.upper_bound.is_some());
        assert!(self.lower_bound.unwrap().len() >= self.variables_count);
        assert!(self.upper_bound.unwrap().len() >= self.variables_count);
    }

    fn working_space(&self) -> Vec<f64> {
        use std::iter::repeat;
        let working_space_size = Bobyqa::working_space_size(
            self.number_of_interpolation_conditions,
            self.variables_count);
        repeat(0.0).take(working_space_size).collect::<Vec<f64>>()
    }

    fn working_space_size(number_of_interpolation_conditions: usize, variables_count: usize) -> usize {
        3*variables_count*(variables_count + 3)/2
        + (number_of_interpolation_conditions + 13)
            *(number_of_interpolation_conditions + variables_count)
    }
}

#[test]
fn test_perform_should_succeed() {
    let mut values = [0.5, 0.5];
    let function = |x: &[f64]| -> f64 { x[0] + x[1] };
    let result = Bobyqa::new().perform(&mut values, &function);
    assert_eq!(values, [0.0, 0.0]);
    assert_eq!(result, 0.0);
}

#[test]
fn test_perform_mut_with_all_settings_should_succeed() {
    let mut calls_count = Box::new(0);
    let mut values = [1.0, -1.0, 0.0];
    let lower_bound = [-3.0, -2.0, -1.0];
    let upper_bound = [1.0, 2.0, 3.0];
    let result = {
        let mut function = |x: &[f64]| -> f64 {
            assert_eq!(x.len(), 3);
            *calls_count += 1;
            x[0] + x[1] + x[2]
        };
        Bobyqa::new()
            .variables_count(values.len())
            .number_of_interpolation_conditions((values.len() + 1)*(values.len() + 2)/2)
            .initial_trust_region_radius(1e-3)
            .final_trust_region_radius(1e3)
            .lower_bound(&lower_bound)
            .upper_bound(&upper_bound)
            .max_function_calls_count(25)
            .perform_mut(&mut values, &mut function)
    };
    assert_eq!(values, lower_bound);
    assert_eq!(result, -6.0);
    assert_eq!(*calls_count, 25);
}
