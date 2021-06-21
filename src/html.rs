/**!
 * HTML writer which is better suited to our needs than the pulldown_cmark one.
 * Mostly taken from pulldown_cmark though.
 */
use pulldown_cmark::{CowStr, Event, LinkType, Tag};
use std::io::{self, Write};

enum TableState {
    Head,
    Body,
}

/// FIXME: take an iterator over (Event, Range)
/// so that errors can be emitted and shown
///
/// FIXME: implement tables and footnotes
/// TODO: detect heading links and slugify them
pub(crate) struct HtmlWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: Write,
{
    iter: I,
    writer: W,

    table_state: Option<TableState>,
}

impl<'a, I, W> HtmlWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: Write,
{
    pub(crate) fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            table_state: None,
        }
    }

    pub(crate) fn run(&mut self) -> io::Result<()> {
        use Event::*;
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => self.start_tag(tag)?,
                End(tag) => self.end_tag(tag)?,
                Text(text) => self.write_text(&text)?,
                Code(text) => self.write_code(&text)?,
                Html(html) => self.write_html(&html)?,
                SoftBreak => self.write_newline()?,
                HardBreak => self.write_break()?,
                Rule => self.write_rule()?,
                TaskListMarker(checked) => self.write_task_list_marker(checked)?,
                _ => todo!("rest of the owl"),
            }
        }
        Ok(())
    }

    fn write_task_list_marker(&mut self, checked: bool) -> io::Result<()> {
        if checked {
            write!(self.writer, "<input disabled type=\"checkbox\" checked />")?;
        } else {
            write!(self.writer, "<input disabled type=\"checkbox\" checked />")?;
        }
        Ok(())
    }

    fn write_rule(&mut self) -> io::Result<()> {
        write!(self.writer, "<hr />")?;
        Ok(())
    }

    fn write_break(&mut self) -> io::Result<()> {
        write!(self.writer, "<br />")?;
        Ok(())
    }

    fn write_newline(&mut self) -> io::Result<()> {
        write!(self.writer, "\n")?;
        Ok(())
    }

    // FIXME: figure out html stuff
    fn write_html(&mut self, html: &str) -> io::Result<()> {
        self.write_text(html)
    }

    fn write_code(&mut self, code: &str) -> io::Result<()> {
        write!(self.writer, "<code>")?;
        self.write_text(code)?;
        write!(self.writer, "</code>")?;
        Ok(())
    }

    // FIXME: escape text
    fn write_text(&mut self, text: &str) -> io::Result<()> {
        write!(self.writer, "{}", text)?;
        Ok(())
    }

    fn write_linkish(
        &mut self,
        tag: &str,
        ty: LinkType,
        dest: CowStr,
        title: CowStr,
    ) -> io::Result<()> {
        write!(self.writer, "<{} href=\"{}", tag, get_link_url(ty, dest))?;
        if !title.is_empty() {
            write!(self.writer, "\" title=\"{}", title)?;
        }
        write!(self.writer, "\">")?;
        Ok(())
    }

    // FIXME: tables and footnote definitions
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        use pulldown_cmark::CodeBlockKind;
        use Tag::*;
        match tag {
            Paragraph => write!(self.writer, "<p>")?,
            Heading(level) => write!(self.writer, "<h{}>", level)?,
            Table(alignments) => todo!(),
            TableHead => todo!(),
            TableRow => todo!(),
            TableCell => todo!(),
            BlockQuote => write!(self.writer, "<blockquote>")?,
            CodeBlock(kind) => match kind {
                CodeBlockKind::Fenced(lang) => {
                    if lang.is_empty() {
                        write!(self.writer, "<pre><code>")?
                    } else {
                        write!(self.writer, "<pre><code class=\"language-{}\">", lang)?
                    }
                }
                CodeBlockKind::Indented => write!(self.writer, "<pre><code>")?,
            },
            List(Some(start)) => write!(self.writer, "<ol start=\"{}\">", start)?,
            List(None) => write!(self.writer, "<ul>")?,
            Item => write!(self.writer, "<li>")?,
            Emphasis => write!(self.writer, "<em>")?,
            Strong => write!(self.writer, "<strong>")?,
            Strikethrough => write!(self.writer, "<del>")?,
            Link(ty, dest, title) => self.write_linkish("a", ty, dest, title)?,
            Image(ty, dest, title) => self.write_linkish("img", ty, dest, title)?,
            FootnoteDefinition(name) => todo!(),
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        use Tag::*;
        match tag {
            Paragraph => write!(self.writer, "</p>")?,
            Heading(level) => write!(self.writer, "</h{}>", level)?,
            Table(..) => todo!(),
            TableHead => todo!(),
            TableRow => todo!(),
            TableCell => todo!(),
            BlockQuote => write!(self.writer, "</blockquote>")?,
            CodeBlock(..) => write!(self.writer, "</code></pre>")?,
            List(Some(_)) => write!(self.writer, "</ol>")?,
            List(None) => write!(self.writer, "</ul>")?,
            Item => write!(self.writer, "</li>")?,
            Emphasis => write!(self.writer, "</em>")?,
            Strong => write!(self.writer, "</strong>")?,
            Strikethrough => write!(self.writer, "</del>")?,
            Link(..) => write!(self.writer, "</a>")?,
            Image(..) => write!(self.writer, "</img>")?,
            FootnoteDefinition(..) => todo!(),
        }
        Ok(())
    }
}

fn get_link_url(ty: LinkType, url: CowStr) -> CowStr {
    match ty {
        LinkType::Email => format!("mailto:{}", url).into(),
        _ => url,
    }
}
