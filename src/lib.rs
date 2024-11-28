use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;

pub struct HideContent;

impl HideContent {
    pub fn new() -> HideContent {
        HideContent
    }
}

impl Preprocessor for HideContent {
    fn name(&self) -> &str {
        "hide-content"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                process_chapter(chapter);
            }
        });
        Ok(book)
    }
}

fn process_chapter(chapter: &mut Chapter) {
    let css = r#"
<style>
.answer-toggle {
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    color: #666;
    padding: 4px 8px;
    font-size: 0.9em;
    margin: 0.5rem 0;
    border: none;
    background: none;
    transition: color 0.2s ease;
}

.answer-toggle:hover {
    color: #333;
}

.toggle-icon {
    margin-right: 6px;
}

.toggle-icon svg {
    width: 16px;
    height: 16px;
    vertical-align: middle;
}

.hidden-content {
    display: none;
    margin-top: 8px;
}

.icon-eye {
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
}

/* 确保不影响 markdown 内容的样式 */
.markdown-content {
    display: block;
}
.markdown-content > :first-child {
    margin-top: 0;
}
.markdown-content > :last-child {
    margin-bottom: 0;
}
</style>
"#;

    let js = r#"
<script>
window.addEventListener('load', function() {
    document.querySelectorAll('.hidden-content').forEach(content => {
        content.style.display = 'none';
    });

    document.querySelectorAll('.answer-toggle').forEach(button => {
        button.addEventListener('click', function() {
            const container = this.closest('.qa-container');
            const content = container.querySelector('.hidden-content');
            const svg = this.querySelector('svg');
            const text = this.querySelector('.toggle-text');

            if (content.style.display === 'none') {
                content.style.display = 'block';
                text.textContent = '隐藏答案';
                svg.innerHTML = `
                    <path class="icon-eye" d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                    <path class="icon-eye" d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                    <path class="icon-eye" d="M3 3l18 18"></path>
                `;
            } else {
                content.style.display = 'none';
                text.textContent = '显示答案';
                svg.innerHTML = `
                    <path class="icon-eye" d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                    <path class="icon-eye" d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                `;
            }
        });
    });
});
</script>
"#;

    // 更新正则表达式以捕获问题文本
    let re = Regex::new(r"(?ms)^(.*?)\n@@@\n(.*?)\n@@@$").unwrap();
    chapter.content = format!("{}\n{}", css, chapter.content);
    chapter.content = format!("{}\n{}", js, chapter.content);

    chapter.content = re.replace_all(&chapter.content, |caps: &regex::Captures| {
        let title = &caps[1];
        let content = &caps[2];

        format!(r#"{title}
<div class="qa-container">
<button class="answer-toggle" aria-label="Toggle answer">
    <span class="toggle-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16">
            <path class="icon-eye" d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
            <path class="icon-eye" d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
        </svg>
    </span>
    <span class="toggle-text">显示答案</span>
</button>
<div class="hidden-content">

{content}

</div>
</div>"#
        )
    }).to_string();
}