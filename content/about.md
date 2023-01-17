```metadata
develop = false
```

# A new blog

It is time.

A few years ago when I was first learning programming I thought often about how useful it would be
to write down the information I was learning from the perspective of a newbie because I _**knew**_
what the actual difficulties were for a beginner.

But, alas, I was too busy learning programming to actually write about it. So now a few years have
gone by, I'm not so much of a begginer anymore and this probably isn't going to be as useful as it
once was... but here we are. And I'm doing it.

## The goal

The goal for this blog is to write about things as I learn them, not create tutorials.

I hope to be able to capture the insights that I'll gain when I first start learning a new thing and
share them in a way that is both informative and entertaining.

In the next year or two I have a large number of goals for things that I would like to learn and
accomplish. Such as...

- Designing a keyboard from scratch. This includes

  - Designing with 3d modeling software the housing
  - Designing the PCB (I'll have someone else print it)
  - Writing the firmware from kind of scratch in rust.

- Building the blog engine that I'll use to make this blog awesome (or at least as aweosme as a
  programming blog can be).

- Continuing to build random cli tools such as my
  [tmux sessionizer](https://github.com/jrmoulton/tmux-sessionizer) and develop other rust
  embedded-hal drivers like my [DACx0501 driver](https://github.com/jrmoulton/dacx0501) and continue
  to work on [esp-hal](https://github.com/esp-rs/esp-hal).

  Ideally having this blog will motivate me to work on and finish larger scale projects as I write
  and incrementally publish what I've worked on.

## Where this blog currently is

Currently this blog is just me writing markdown files in Neovim. I'm not super satisfied with the
current offerings for static site generation such as Hugo and zola and I think I want to build a
custom engine similar to what [Amos/fasterthanlime](https://fasterthanli.me) has. Mostly I want to
do this because I want more control over syntax highlighting

```date
    August 15, 2022
```

## A jump to the future

Wow! Check out this blog! It's come quite a ways since I wrote that I was just writing markdown in
neovim clear back in August.

I mean, look at me know! I'm still writing markdown!

But it looks prettier!

For example...

```rust
fn write_articles(templ_reg: &mut Handlebars, themes: &mut Themes) -> LatestArticles {
    let files: Vec<_> = std::fs::read_dir("./content/")
        .expect("couldnt read the content directory")
        .collect();

    let theme =
        tree_painter::Theme::from_helix(include_str!("../../../themes/onedark_dark.toml")).unwrap();

    let latest_articles = Arc::new(Mutex::new(LatestArticles::new()));

    files.par_iter().for_each(|file| {

        let mut date = dateparser::parse("1/1/2000").unwrap();

        let article_string = file.as_ref().unwrap();

        let input = std::fs::read_to_string(article_string.path()).unwrap();

        let mut renderer = tree_painter::Renderer::new(theme.clone());

        ...

        }
    );
}
```

This is a bit of code from the generator that renders each article in parallel (it's blazingly fast
by the way). And that syntax highlighting looks awesome right!?! And it's fully responsive to the
the website theme.

Defnitely one of the first articles that I write in the near future will be how I built this static
site generator and all of the things I've been figuring out. I've learned how to generate all the
things, format them and get some pretty good theming going! (check out the theme button...)

I'll be writing about these things as I learn them because, again, the whole point is to capture the
ideas while they are fresh. So follow me on [twitter](https://twitter.com/jrmoulton3)! I'm
@jrmoulton3 over there.

See you back here soon!

```date
    December 14, 2022
```
