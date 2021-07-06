//#![deny(unused_imports)]
//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;

use panic_halt as _;

use nb::block;
use rtic::app;

//use cortex_m_semihosting::{debug, hprintln};

//use cortex_m::asm;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    pac::TIM2,
    timer::{Timer, CountDownTimer},
    gpio::{gpioc::PC13, Output, PushPull, State},
    prelude::*,
};

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        #[init(1.hz())]
        freq: i32,
        LED: PC13<Output<PushPull>>,
        //static mut DELAY: Delay                 = ();
        TIMER: CountDownTimer<TIM2>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources{
        // Get access to the core peripherals from the cortex-m crate
        //let cp = cortex_m::Peripherals::take().unwrap();
        // Get access to the device specific peripherals from the peripheral
        // access crate
        //let dp = Peripherals::take().unwrap();
        // Take ownership over the raw flash and rcc devices and convert them
        // into the corresponding HAL structs
        let mut rcc = cx.device.RCC.constrain();
        let mut flash = cx.device.FLASH.constrain();

        let _afio = cx.device.AFIO.constrain(&mut rcc.apb2);
        let _channels = cx.device.DMA1.split(&mut rcc.ahb);

        let _gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        // Freeze the configuration of all the clocks in the system and store the
        // frozen frequencies in clocks
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let t = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1);

        let _pump1 = gpiob.pb8
            .into_push_pull_output_with_state(&mut gpiob.crh, State::High);
        let _pump2 = gpiob.pb9
            .into_push_pull_output_with_state(&mut gpiob.crh, State::Low);


        //hprintln!("init").unwrap();
        // Configure gpio C pin 13 as a push-pull output.
        // The `crh` register is passed to the function in order to configure 
        // the port.
        // For pins 0-7, crl should be passed instead.
        init::LateResources {
            LED: gpioc.pc13.into_push_pull_output_with_state(&mut gpioc.crh, State::High),
            TIMER: t.start_count_down(1.hz()),
        }
    }

    #[idle(resources = [LED, TIMER])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            block!((cx.resources.TIMER).wait()).unwrap();
            (cx.resources.LED).set_high().unwrap();
            //block!((cx.resources.TIMER).wait()).unwrap();
            //(cx.resources.LED).set_low().unwrap();
        }
    }
};
