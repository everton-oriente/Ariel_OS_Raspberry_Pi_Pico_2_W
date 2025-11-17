use ariel_os::hal::{peripherals, spi, i2c};

// Input Configurations pins
#[cfg(context = "rp235xa")]
ariel_os::hal::define_peripherals!(InputPeripherals {
    switch_a: PIN_12, // Switch A
    switch_b: PIN_13, // Switch B
    switch_x: PIN_14, // Switch X
    switch_y: PIN_15, // Switch Y
});

// Output Configurations pins
#[cfg(context = "rp235xa")]
ariel_os::hal::define_peripherals!(OutputPeripherals {
    pin_2: PIN_2,
    pin_3: PIN_3,
    pin_4: PIN_4,
    pin_5: PIN_5,
    pin_6: PIN_6,
    pin_7: PIN_7,
    motor1_pwm_pos: PIN_8,
    motor1_pwm_neg: PIN_9,
    motor2_pwm_pos: PIN_10,
    motor2_pwm_neg: PIN_11,
});

// ADC Input Configurations pins
#[cfg(context = "rp235xa")]
ariel_os::hal::define_peripherals!(AdcPeripherals {
    adc_0: PIN_26, // ADC Channel 0
    adc_1: PIN_27, // ADC Channel 1
    adc_2: PIN_28, // ADC Channel 2
});

// Spi Configurations pins
#[cfg(context = "rp235xa")]
pub type SensorSpi = spi::main::SPI0;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(SpiPeripherals {
    spi_sck: PIN_18,
    spi_miso: PIN_16,
    spi_mosi: PIN_19,
    spi_cs: PIN_17,
});

// I2c Configurations pins
#[cfg(context = "rp235xa")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(I2cPeripherals {
    i2c_sda: PIN_20,
    i2c_scl: PIN_21,
});

/*
// Uart Configurations pins
#[cfg(context = "rp235xa")]
pub type Uart<'a> = uart::UART0<'a>;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(UartPeripherals {
    uart_rx: PIN_1,
    uart_tx: PIN_0,
});
*/







