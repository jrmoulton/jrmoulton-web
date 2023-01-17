```metadata
develop = false
```

# My experience starting with open source

Ultimately, this article is the culprit.

It isn't guilty of forcing me to to start my open source work, or even to write about it... but
rather of making me spend a ton of time building a site generator to make it look pretty

And boy does it look pretty.

But that's not what this article is about.

This is the article that made me want to actually put in the effort to create this blog (the content
I mean, not necessarily the the site generator) and this is the fateful story.

## How it happened

Recently(ish) I decided to contribute to some open source projects and I took the advice I had been
given on youtube for getting started (cause how else am I supposed to learn anything) with open
source and I made a contribution to... Drum Rooollll...

Yep! You guessed it!

the documentation.

Kind of underwhelming, I know.

But it was definitely a good way to get started with open source work. It didn't cause me stress
worrying if it was high enough quality and I leared how to actually open and work a pull request.

The image below is the first PR that I ever opened. It wasn't perfect (later corrections were made)
and it wasn't anything too special but it was honest work.
![Time First PR](/images/first_pr_top.png) Basically when I was using the slint (then sixtyfps)
framework I found a part of the documentation that was confusing to me and I opened a PR that made
some clarifications.

### The result!

![First PR result](/images/first_pr_middle.png) Woooohoooo!!!!

I was crazy excited for this PR to be merged. I know it seems kinda insignificant but to me it was
an awesome moment.

I probably bragged to my wife about my awesome changes to the docs for long enough to get some funny
looks

#### And some corrections...

![First PR Corrections](/images/first_pr_bottom.png)

The slint team members were super nice to work with

### So... why docs??

For me, this has two major benefits. One is that I can get a feel for how that particular project is
run. Maybe they have a particular merge strategy and they would prefer if all of the commits are
squashed. Or maybe something crazy! Who knows!?! There are process details like this that I feel are
nice to get out of the way with low pressure.

- These process details are often talked about in a CONTRIBUTING.md file but at the time I didn't
  really understand what the heck it was talking about until I had actually seen those processes it
  in my own PRs.

But the second is probably the bigger reason

I don't want to spend a chunk of my time working on a project when the maintainers are hostile. A
small change to the docs doesn't take too much time and I know I wont feel hurt (relatively) if it
essentially becomes wasted time. This has never happened to me and it doesn't seem common but I have
seen it. I defnitely don't want to have to deal with that.

## But why at all?

I decided to start contributing to open source projects because I was going to be applying for
summer internships and I was hoping that open source contributions would look good on a resume and
in interviews.

And that was honestly the main motivation for the first PR.

Since then I have found that I genuinely enjoy the challenge of working on new projects and
unfamiliar codebases. I enjoy seeing other people use and improve on my changes and it is definitely
a great way to learn and get exposure to a variety of codebases.

## What was the biggest challenge in getting started?

### Fear.

That sums it up pretty well.

I was very afraid of doing something that was just an inconvenience to other developers.<br> I was
afraid of implementing something incorrectly.<br> I was afraid that I would screw up the
process.<br> I was afraid of being being out of place.<br>

Luckily, the slint developers were super kind and helpful and made my first experiences with open
souce work an awesome experience that made me want to do it again.

Also I felt lucky that there was high quality content on youtube that showed the process of opening
a PR and working with git to the point that I thought...

> Yeah. I can do that.

## What have been the results

### More work with slint

Since that first PR I've been able to open a few other PR's on the slint project but on these I
actually dug into the code.

When I was working on these PRs I still had no idea how to grok a codebase that I hadn't personally
written **_and_** I was fairly new to Rust at the time.

Slowly, bit by bit (haha), I was able to find my way around the codebase and find the relevant parts
of the code.

- Funny Side note: This was probably the first time that I really realized how useful "Goto
  Definition" could be.

One of the code changes that I made that was actually merged was adding some built-in functions to
the slint language.

```rust
    Expression::BuiltinFunctionReference(BuiltinFunction::Log, _) => {
        let x: f64 = eval_expression(&arguments[0], local_context).try_into().unwrap();
        let y: f64 = eval_expression(&arguments[1], local_context).try_into().unwrap();
        Value::Number(x.log(y))
    }
    Expression::BuiltinFunctionReference(BuiltinFunction::Pow, _) => {
        let x: f64 = eval_expression(&arguments[0], local_context).try_into().unwrap();
        let y: f64 = eval_expression(&arguments[1], local_context).try_into().unwrap();
        Value::Number(x.powf(y))
    }
```

This was the largest essential change in the PR where the functionality for `pow` and `log`
functions were added to be language built-ins.

- Looking back there are things I would change for this PR (mostly involving better error handling).

And again, the slint maintainers were awesoeme to work with and answer my questions. <br>#notanad

After getting this work done something magical happened...

### Work with ESP-HAL

My work with esp-hal started after I got my first internship!

Thank you, thank you. You're too kind. Hold the applause please.

Haha jokes aside I was super happy to have my first internship. I was working as a firmware
engineering intern in C for some internal company tools. It was an awesome opportunity because it
was a brand new project that myself and another itern built mostly from scratch.

While I was working there I started to really fall in love with embedded development and,
simultaneously, I wanted to add some more Rust to my life.

So that's what I did.

I tried getting an LED to blink using a microcontroller that had no existing support in rust.

And wow. I did know what I was in for

It was tons of time

- digging through datasheets
- tracking down svd files
- learning a whole new world of embedded rust
- digging through datasheets
- learning how microcontroller cpu's control peripherals (such as GPIO pins)
- and still digging through datasheets

I spent a very long time trying to figure out how to get support for it and one day it worked!!!

The LED BLINKED!!!!

It was probably in the top 3 most magical moments of my life.

I had blinked LED's before using an arduino but **_I_** had made this thing work by following a
datasheet and building periphal access crates and writing to registers and AN LED BLINKED!!!

I probably just ranted about how cool it was for at least 2 hours. After saying "IT WORKS!!" and
"THE LED BLINKS" for probably the 300th time I knew my wife must truly love me because she was still
replying with "Wow. Yeah that's really cool."

Unfortunately I can't find the original code that I used to blink the led and figuring it out again
would involve digging into the datasheet to see exactly what bits need to be set but it would have
started about like this (after finally generating a peripheral access crate from an svd file)

```rust
#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = psoc4_pac::Peripherals::take().unwrap();
    peripherals.GPIO.prt1.pc.write(|w| unsafe { w.dm3().bits(6) });
    peripherals.GPIO.prt1.dr.write(|w| w.data3().set_bit());

    loop {
        // delay and blinking goes here
    }
}

```

This code gets a global singleton of the peripherals and then sets both a configuration register to
control the pin drive mode and sets another register with the data to set the pin high.

All this work led me to find projects that had already built out a lot of support for chips in rust
in the form of "Hardware Abstraction Layers" or "HALS". So far my favorite of these is the ESP-HAL.

Currently rust is a super exciting language for writing embedded code because of a project called
[embedded-hal](https://github.com/rust-embedded/embedded-hal) that defines a set of common traits
that allow device drivers to be hardware independent!

And that is **_crazy_** cool because previously any time you wanted to use an external peripheral
you had to write a custom driver for your target microcontroller to interface with the device. But
now it really is as easy as writing the device driver once and using it on every microcontroller.

Now, there is an awesome series of fairly cheap but powerful microcontrollers from espressif... and
rust is awesome...

So back to the purpose of this section...

### ESP-HAL.

ESP-HAL is rust support for espressif microcontrollers. It's open source and sponsored by espressif
with developers actively working on the project.

I was building a device driver for a digital to analog converter
[DACx0501](https://github.com/jrmoulton/dacxo501) and I worked a bit on the SPI support in ESP-HAL.

```rust
#[cfg(feature = "eh1")]
 impl<T> embedded_hal_1::spi::blocking::SpiBusWrite for Spi<T>
 where
     T: Instance,
 {
     fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
         self.spi.send_bytes(words)
     }
 }

 #[cfg(feature = "eh1")]
 impl<T> embedded_hal_1::spi::blocking::SpiBusFlush for Spi<T>
 where
     T: Instance,
 {
     fn flush(&mut self) -> Result<(), Self::Error> {
         self.spi.flush()
     }
 }
```

This code is the main driver code that implements the embedded_hal traits and calls out to other
functions.

Essentially it allows users of the HAL to call two functions.

The `write` function takes in any stream of bytes and transfers them over the SPI protocol using the
device's built in hardware SPI controller.

The `flush` function ensures that the bus is free (allowing the user to know that the transfer has
finished).

And the implementation

```rust
    fn send_bytes(&mut self, words: &[u8]) -> Result<(), Infallible> {
         let reg_block = self.register_block();
         let num_chuncks = words.len() / 64;

         for (i, chunk) in words.chunks(64).enumerate() {
             self.configure_datalen(chunk.len() as u32 * 8);

             let mut fifo_ptr = reg_block.w0.as_ptr();
             for chunk in chunk.chunks(4) {
                 let mut u32_as_bytes = [0u8; 4];
                 unsafe {
                     let ptr = u32_as_bytes.as_mut_ptr();
                     ptr.copy_from(chunk.as_ptr(), chunk.len());
                 }
                 let reg_val: u32 = u32::from_le_bytes(u32_as_bytes);

                 unsafe {
                     *fifo_ptr = reg_val;
                     fifo_ptr = fifo_ptr.offset(1);
                 };
             }

             self.update();

             reg_block.cmd.modify(|_, w| w.usr().set_bit());

             // Wait for all chunks to complete except the last one.
             // The function is allowed to return before the bus is idle.
             // see [embedded-hal flushing](https://docs.rs/embedded-hal/1.0.0-alpha.8/embedded_hal/spi/blocking/index.html#flushing)
             if i < num_chuncks {
                 while reg_block.cmd.read().usr().bit_is_set() {
                     // wait
                 }
             }
         }
         Ok(())
     }

     // Check if the bus is busy and if it is wait for it to be idle
     fn flush(&mut self) -> Result<(), Infallible> {
         let reg_block = self.register_block();

         while reg_block.cmd.read().usr().bit_is_set() {
             // wait for bus to be clear
         }
         Ok(())
     }
```

If you think this code looks impressive I'll feel better about the rather long amount of time that
it took me to write it...

It takes the stream of bytes and chucks them in 64 byte chuncks and then fills the devices registers
with that data and then sets a flag bit on a control register that allows the device to start
sending data that is in the registers.

- The code I wrote was later improved on

Oh, and then it totally worked for me!

I used it to test/finish implementing my driver for the digital to analog converter.

## My own projects

I guess I can also count my personal projects as open source... since they are... open source.

### tree-sitter-slint

More slint stuff!!

This was just a project that I started as a quality of life improvement when I was frustrated by the
complete lack of color in the slint language.

Tree-sitter is a tool that allows you to write a language gramer in javascript and is primarily used
for syntax highliting on code (it's what I'm using to highlight the code on this website).

So I wrote a grammar that handles slint code!

(The code below isn't fully functional. Just meant to show off the syntax highlighting)

```slint

MemoryTile := Rectangle {
    border-radius: 8px;
    callback clicked;
    callback some-callback;
    property <bool> open-curtain;
    property <bool> solved;
    property <image> icon;

    background: solved ? #70ff00 : #858585;
    animate background { duration: 800ms; }

    clicked => {
        if (condition) {
            foo = 42;
        } else if (other-condition) {
            bar = 28;
        } else {
            foo = 4;
        }
    }
    // Left curtain
    Rectangle {
        background: #0025ff;
        border-radius: 4px;
        width: open-curtain ? 0px : parent.width / 2 + 4px;
        height: parent.height;
        animate width { duration: 250ms; easing: ease-in; }
        clip: true;

        Image {
            width: root.width - 32px;
            height: root.height - 32px;
            x: 16px;
            y: 16px;
            source: @image-url("icons/tile_logo.png");
        }
    }

    states [
        disabled when !is-enabled : {
            color: gray; // same as root.color: gray;
            root.color: white;
        }
        down when pressed : {
            background: blue;
        }
    ]

    transitions [
        out disabled : {
            animate * { duration: 800ms; }
        }
        in down : {
            animate color { duration: 300ms; }
        }
    ]
}
```

Vs what it basically looked like before a proper grammar

```css
MemoryTile := Rectangle {
    border-radius: 8px;
    callback clicked;
    callback some-callback;
    property <bool> open-curtain;
    property <bool> solved;
    property <image> icon;

    background: solved ? #70ff00 : #858585;
    animate background { duration: 800ms; }

    clicked => {
        if (condition) {
            foo = 42;
        } else if (other-condition) {
            bar = 28;
        } else {
            foo = 4;
        }
    }
    // Left curtain
    Rectangle {
        background: #0025ff;
        border-radius: 4px;
        width: open-curtain ? 0px : parent.width / 2 + 4px;
        height: parent.height;
        animate width { duration: 250ms; easing: ease-in; }
        clip: true;

        Image {
            width: root.width - 32px;
            height: root.height - 32px;
            x: 16px;
            y: 16px;
            source: @image-url("icons/tile_logo.png");
        }
    }

    states [
        disabled when !is-enabled : {
            color: gray; // same as root.color: gray;
            root.color: white;
        }
        down when pressed : {
            background: blue;
        }
    ]

    transitions [
        out disabled : {
            animate * { duration: 800ms; }
        }
        in down : {
            animate color { duration: 300ms; }
        }
    ]
}
```

(This is actually just trying to highlight this as css but it's pretty close to what it looked like
before)

- Dang. I've got a thing for good syntax highlighting.

## Closing thoughts

I keep learning a ton by contributing to open source projects and I don't plan on stopping.

But even with everything I'm learning I'm highly suspicious that in the future any new project I
contribute to will probably first receive a new contributor to the documentation.

```date
    1/1/2023
```
