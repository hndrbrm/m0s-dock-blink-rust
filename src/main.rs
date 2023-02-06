#![no_std]
#![no_main]

use panic_halt as _;

const GPIO_MODE_SHIFT: u32 = 5;
const GPIO_MODE_MASK: u32 = 3 << GPIO_MODE_SHIFT;
const GPIO_OUTPUT: u32 = 1 << GPIO_MODE_SHIFT; /* Output Enable */

const GPIO_PUPD_SHIFT: u32 = 7; /* Bits 7-8: Pull-up/down */
const GPIO_PULLUP: u32 = 1 << GPIO_PUPD_SHIFT; /* Pull-up */

const GPIO_SMT_SHIFT: u32 = 9; /* Bits 9: SMT Enable */
const GPIO_SMT_EN: u32 = 1 << GPIO_SMT_SHIFT;

const GLB_BASE: u32 = 0x2000_0000;
const GLB_GPIO_CFG0_OFFSET: u32 = 0x8c4;
const GLB_GPIO_CFG138_OFFSET: u32 = 0xAEC;/* gpio_cfg138 */
const GLB_GPIO_CFG140_OFFSET: u32 = 0xAF4;/* gpio_cfg140 */

fn bflb_gpio_init(pin: u32, cfgset: u32) {
    const GLB_REG_GPIO_0_SMT: u32 = 1 << 1;
    const GLB_REG_GPIO_0_DRV_SHIFT: u32 = 2;
    const GLB_REG_GPIO_0_PU: u32 = 1 << 4;
    const GLB_REG_GPIO_0_OE: u32 = 1 << 6;
    const GLB_REG_GPIO_0_FUNC_SEL_SHIFT: u32 = 8;
    const GLB_REG_GPIO_0_INT_MASK: u32 = 1 << 22;
    const GLB_REG_GPIO_0_MODE_SHIFT: u32 = 30;

    const GPIO_DRV_SHIFT: u32 = 10; /* Bits 10-11: Drive */
    const GPIO_DRV_MASK: u32 = 3 << GPIO_DRV_SHIFT;

    const GPIO_FUNC_SHIFT: u32 = 0; /* Bits 0-4: GPIO function */
    const GPIO_FUNC_MASK: u32 = 0x1f << GPIO_FUNC_SHIFT;

    let mut function: u32 = (cfgset & GPIO_FUNC_MASK) >> GPIO_FUNC_SHIFT;
    let mode: u32 = cfgset & GPIO_MODE_MASK;
    let drive: u32 = (cfgset & GPIO_DRV_MASK) >> GPIO_DRV_SHIFT;

    let cfg_address: u32 = GLB_BASE + GLB_GPIO_CFG0_OFFSET + (pin << 2);

    let mut cfg: u32 = 0;
    cfg |= GLB_REG_GPIO_0_INT_MASK;

    if mode == GPIO_OUTPUT {
        cfg |= GLB_REG_GPIO_0_OE;
        function = 11;
    }

    if (cfgset & GPIO_PULLUP) > 0 {
        cfg |= GLB_REG_GPIO_0_PU;
    }

    if (cfgset & GPIO_SMT_EN) > 0 {
        cfg |= GLB_REG_GPIO_0_SMT;
    }

    cfg |= drive << GLB_REG_GPIO_0_DRV_SHIFT;
    cfg |= function << GLB_REG_GPIO_0_FUNC_SEL_SHIFT;

    /* configure output mode:set and clr mode */
    cfg |= 0x1 << GLB_REG_GPIO_0_MODE_SHIFT;

    unsafe {
        let cfg_address = cfg_address as *mut u32;
        cfg_address.write_volatile(cfg);
    }
}

fn bflb_gpio_set(pin: u32) {
    let address: u32 = GLB_BASE + GLB_GPIO_CFG138_OFFSET + ((pin >> 5) << 2);

    unsafe {
        let address = address as *mut u32;
        address.write_volatile(1 << (pin & 0x1f));
    }
}

fn bflb_gpio_reset(pin: u32) {
    let address: u32 = GLB_BASE + GLB_GPIO_CFG140_OFFSET + ((pin >> 5) << 2);

    unsafe {
        let address = address as *mut u32;
        address.write_volatile(1 << (pin & 0x1f));
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    const GPIO_PIN_27: u32 = 27;
    const GPIO_PIN_28: u32 = 28;

    const GPIO_DRV_SHIFT: u32 = 10; /* Bits 10-11: Drive */
    const GPIO_DRV_0: u32 = 0 << GPIO_DRV_SHIFT;

    bflb_gpio_init(GPIO_PIN_27, GPIO_OUTPUT | GPIO_PULLUP | GPIO_SMT_EN | GPIO_DRV_0);
    bflb_gpio_init(GPIO_PIN_28, GPIO_OUTPUT | GPIO_PULLUP | GPIO_SMT_EN | GPIO_DRV_0);

    bflb_gpio_set(GPIO_PIN_27);
    bflb_gpio_reset(GPIO_PIN_28);

    loop {}
}
