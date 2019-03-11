#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

// SensorTag is using RGZ package. VQFN (RGZ) | 48 pins, 7×7 QFN

use cc2650_hal as hal;
use cc2650f128;
use cortex_m_rt::entry;

use hal::{ddi, delay::Delay, prelude::*};

pub fn init() -> (Delay, cc2650f128::Peripherals) {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = cc2650f128::Peripherals::take().unwrap();

    let clocks = ddi::CFGR {
        sysclk: Some(24_000_000),
    }
    .freeze();

    let delay = Delay::new(core_peripherals.SYST, clocks);

    // By default GPIOs are configured as inputs. Default output type is push/pull so
    // just need to disable input, and enable output

    // #define Board_STK_LED1              IOID_10
    // #define Board_STK_LED2              IOID_15
    // Board_STK_LED1   | PIN_GPIO_OUTPUT_EN | PIN_GPIO_LOW | PIN_PUSHPULL | PIN_DRVSTR_MAX,     [> LED initially off             <]
    // Board_STK_LED2   | PIN_GPIO_OUTPUT_EN | PIN_GPIO_LOW | PIN_PUSHPULL | PIN_DRVSTR_MAX,     [> LED initially off             <]

    // Lets configure some pins, in the IOC module
    // • PORTID is the number for a peripheral function.
    // • GPIO is a peripheral function with the PORTID of 0x0.
    // • DIO (DIO0 to DIO31) are the logic names for the different I/O pins on the specific package.
    //
    // The PORTID and pin configuration must be set in the corresponding IOC:IOCFGn register. To
    // select what kind of function the pin must be routed, choose the PORTID number for the
    // desired peripheral function and write the PORTID number to the IOC:IOCFGn.PORTID bit field.
    //
    // The MCU GPIO is a general-purpose input/output that drives a number of physical I/O pads.
    // GPIO supports up to 31 programmable I/O pins. These pins are configured by the IOC module.
    //
    // To modify a single GPIO output value, use the GPIO:DOUTn registers (see Section 11.11.2). To
    // set up DIO1 as a GPIO output and toggle the bit, use the following procedure.
    //
    // 1. Map DIO1 as a GPIO output by setting the IOC:IOCFG1.PORT_ID register to 0 (GPIO PORDTID).
    // 2. Ensure DIO1 is set as output by clearing the IOC:IOCFG1.IE bit. More port configurations
    //    can also be set in the IOC:IOCFG1 register (for more details, see Section 11.10.1.2).
    // 3. Set the data output enable bit for DIO1 in GPIO:DOE31_0.DIO1 by issuing a
    //    read-modify-write operation.

    // Configure GPIO pins for output, maximum strength
    device_peripherals.IOC
        .iocfg10
        .modify(|_r, w| w.port_id().gpio().ie().clear_bit().iostr().max());
    device_peripherals.IOC
        .iocfg15
        .modify(|_r, w| w.port_id().gpio().ie().clear_bit().iostr().max());

    // Enable the PERIPH power domain and wait for it to be powered up
    device_peripherals.PRCM.pdctl0.modify(|_r, w| w.periph_on().set_bit());
    loop {
        if device_peripherals.PRCM.pdstat0.read().periph_on().bit_is_set() {
            break;
        }
    }

    // Enable the GPIO clock
    device_peripherals.PRCM.gpioclkgr.write(|w| w.clk_en().set_bit());

    // Load settings into CLKCTRL and wait for LOAD_DONE
    device_peripherals.PRCM.clkloadctl.modify(|_r, w| w.load().set_bit());
    loop {
        if device_peripherals.PRCM.clkloadctl.read().load_done().bit_is_set() {
            break;
        }
    }

    // Enable outputs
    device_peripherals.GPIO
        .doe31_0
        .modify(|_r, w| w.dio10().set_bit().dio15().set_bit());

    (delay, device_peripherals)
}

#[entry]
fn entry() -> ! {
    let (mut delay, periphs) = init();
    let half_period = 500_u16;

    loop {
        // Turn LED on and wait half a second
        periphs.GPIO.dout11_8.modify(|_r, w| w.dio10().set_bit());
        delay.delay_ms(half_period);

        // Turn LED off and wait half a second
        periphs.GPIO.dout11_8.modify(|_r, w| w.dio10().clear_bit());
        delay.delay_ms(half_period);
    }
}
