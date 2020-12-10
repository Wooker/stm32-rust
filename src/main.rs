#![no_main]
#![no_std]

extern crate cortex_m;

use panic_halt as _;

use nb::block;
use rtic::app;

use cortex_m::asm;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    pac::TIM2,
    timer::{Timer, CountDownTimer},
    gpio::{gpioc::PC13, Output, PushPull, State},
    serial::{Serial, Config},
    prelude::*,
};

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        LED: PC13<Output<PushPull>>,
        //static mut DELAY: Delay                 = ();
        TIMER: CountDownTimer<TIM2>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources{
        // Get access to the core peripherals from the cortex-m crate
        //let cp = cortex_m::Peripherals::take().unwrap();
        // Get access to the device specific peripherals from the peripheral access crate
        //let dp = Peripherals::take().unwrap();
        // Take ownership over the raw flash and rcc devices and convert them into the
        // corresponding HAL structs
        let mut rcc = cx.device.RCC.constrain();
        let mut flash = cx.device.FLASH.constrain();

        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);
        let channels = cx.device.DMA1.split(&mut rcc.ahb);

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        // Freeze the configuration of all the clocks in the system and store the 
        // frozen frequencies in clocks`
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        //DELAY = Delay::new(cp.SYST, clocks);
        //DELAY = Delay::new(device.STK, clocks);
        let t = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1);

        /*
        let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let rx = gpioa.pa10;


        let serial = Serial::usart1(
            cx.device.USART1,
            (tx, rx),
            &mut afio.mapr,
            Config::default().baudrate(9600.bps()),
            clocks,
            &mut rcc.apb2,
        );

        //let tx = serial.split().0.with_dma(channels.4);

        //tx.write(b"The quick brown fox").wait();
        //asm::bkpt();
        */
        let pump1 = gpiob.pb8.into_push_pull_output_with_state(&mut gpiob.crh, State::High);
        let pump2 = gpiob.pb9.into_push_pull_output_with_state(&mut gpiob.crh, State::Low);

        // Configure gpio C pin 13 as a push-pull output.
        // The `crh` register is passed to the function in order to configure the port.
        // For pins 0-7, crl should be passed instead.
        init::LateResources {
            LED: gpioc.pc13.into_push_pull_output_with_state(&mut gpioc.crh, State::High),
            TIMER: t.start_count_down(1.hz()),
        }
    }

    #[idle(resources = [LED, TIMER])]
    fn idle(cx: idle::Context) -> ! {
        /*{{{
        loop {
            for ch in message.chars() {
                let letter = Letter::new(ch);

                // TODO: duration returns the total amount of units per letter.
                // It should return the total amount of units per unit of Code.
                for i in letter.code().sequence().chars() {
                    led.set_low().unwrap();
                    match i {
                        '.' => {
                            block!(timer.wait()).unwrap();

                            led.set_high().unwrap();
                            block!(timer.wait()).unwrap();
                        }
                        '-' => {
                            block!(timer.wait()).unwrap();
                            block!(timer.wait()).unwrap();
                            block!(timer.wait()).unwrap();

                            led.set_high().unwrap();
                            block!(timer.wait()).unwrap();
                        }
                        ' ' => {
                            led.set_high().unwrap();
                            block!(timer.wait()).unwrap();
                            block!(timer.wait()).unwrap();
                            block!(timer.wait()).unwrap();
                        }
                        _ => (),
                    };

                }
            }
            block!(timer.wait()).unwrap();
            block!(timer.wait()).unwrap();
            block!(timer.wait()).unwrap();
            block!(timer.wait()).unwrap();
            block!(timer.wait()).unwrap();
            block!(timer.wait()).unwrap();
        }
        }}}*/
        loop {
            //block!((*resources.TIMER).wait()).unwrap();
            //(*resources.DELAY).delay_ms(1_000_u16);
            block!((cx.resources.TIMER).wait()).unwrap();
            (cx.resources.LED).set_high().unwrap();
            block!((cx.resources.TIMER).wait()).unwrap();
            (cx.resources.LED).set_low().unwrap();
        }
    }
};
