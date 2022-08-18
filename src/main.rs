#![no_std]
#![no_main]

use rp_pico::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

use rp_pico::hal::{
  clocks::{init_clocks_and_plls, Clock},
  pac,
  sio::Sio,
  watchdog::Watchdog,
  pio::PIOExt,
  gpio::{FunctionPio0, Pin}
};

fn frame(x: u32) -> u32 {
  let y: u32 = (1 << 10) + ((((x>>7) ^ (x>>6) ^ (x>>5) ^ (x>>4) ^ (x>>3) ^ (x>>2) ^ (x>>1) ^ (x)) & 1 ^ 1) << 9) + (x << 1);
  info!("tx {:x}", x);
  debug!("{:011b}",  y);
  return y;
}

#[entry]
fn main() -> ! {
  info!("Program start");
  let mut pac = pac::Peripherals::take().unwrap();
  let core = pac::CorePeripherals::take().unwrap();
  let mut watchdog = Watchdog::new(pac.WATCHDOG);
  let sio = Sio::new(pac.SIO);
  
  let external_xtal_freq_hz = 12_000_000u32;
  let clocks = init_clocks_and_plls(
    external_xtal_freq_hz,
    pac.XOSC,
    pac.CLOCKS,
    pac.PLL_SYS,
    pac.PLL_USB,
    &mut pac.RESETS,
    &mut watchdog,
  )
  .ok()
  .unwrap();
  
  let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
  
  let pins = rp_pico::Pins::new(
    pac.IO_BANK0,
    pac.PADS_BANK0,
    sio.gpio_bank0,
    &mut pac.RESETS,
  );
  
  let _: Pin<_, FunctionPio0> = pins.gpio11.into_mode();
  let _: Pin<_, FunctionPio0> = pins.gpio12.into_mode();
  let _: Pin<_, FunctionPio0> = pins.gpio14.into_mode();
  let _: Pin<_, FunctionPio0> = pins.gpio15.into_mode();
  
  pins.gpio13.into_push_pull_output().set_high().unwrap();
  
  let mut led_pin = pins.led.into_push_pull_output();
  
  let mut a = pio::Assembler::<{ pio::RP2040_MAX_PROGRAM_SIZE }>::new();
  let mut wrap_target = a.label();
  let mut wrap = a.label();
  
  a.bind(&mut wrap_target);
  a.set(pio::SetDestination::PINS, 1);
  a.out(pio::OutDestination::PINS, 1);
  a.set_with_delay(pio::SetDestination::PINS, 0, 1);
  a.bind(&mut wrap);
  
  let program = a.assemble_with_wrap(wrap, wrap_target);
  let (mut pio, smkbd, smms, _, _) = pac.PIO0.split(&mut pac.RESETS);
  let installed = pio.install(&program).unwrap();
  
  let (mut sm1, _, mut txkbd) = rp_pico::hal::pio::PIOBuilder::from_program(installed)
     .buffers(rp_pico::hal::pio::Buffers::OnlyTx)
     .set_pins(11, 1)
     .out_pins(12, 1)
     .autopull(true)
     .out_shift_direction(rp_pico::hal::pio::ShiftDirection::Right)
     .pull_threshold(11)
     .clock_divisor(2560f32)
     .build(smkbd);
  sm1.set_pindirs([(11, rp_pico::hal::pio::PinDir::Output), (12, rp_pico::hal::pio::PinDir::Output)]);
  sm1.start();
  
  let installed = pio.install(&program).unwrap();
  
  let (mut sm2, _, mut txms) = rp_pico::hal::pio::PIOBuilder::from_program(installed)
     .buffers(rp_pico::hal::pio::Buffers::OnlyTx)
     .set_pins(14, 1)
     .out_pins(15, 1)
     .autopull(true)
     .out_shift_direction(rp_pico::hal::pio::ShiftDirection::Right)
     .pull_threshold(11)
     .clock_divisor(2560f32)
     .build(smms);
  sm2.set_pindirs([(14, rp_pico::hal::pio::PinDir::Output), (15, rp_pico::hal::pio::PinDir::Output)]);
  sm2.start();
  
  loop {
    led_pin.set_high().unwrap();
    txkbd.write(frame(0x12)); delay.delay_ms(2);
    txkbd.write(frame(0x2c)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x2c)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x12)); delay.delay_ms(2);
    
    txkbd.write(frame(0x24)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x24)); delay.delay_ms(2);
    
    txkbd.write(frame(0x1b)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x1b)); delay.delay_ms(2);
    
    txkbd.write(frame(0x2c)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x2c)); delay.delay_ms(2);
    
    txkbd.write(frame(0x5a)); delay.delay_ms(2);
    txkbd.write(frame(0xf0)); delay.delay_ms(2);
    txkbd.write(frame(0x5a)); delay.delay_ms(2);
    
    txms.write(frame(0x09)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    
    led_pin.set_low().unwrap();
    delay.delay_ms(1000);
    
    led_pin.set_high().unwrap();
    
    txms.write(frame(0x08)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    txms.write(frame(0x00)); delay.delay_ms(2);
    
    led_pin.set_low().unwrap();
    delay.delay_ms(1000);
  }
}
