use yew::prelude::*;
use web_sys::window;

pub struct Contact {}

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="tool-wrapper ver2">
                    <div>
                        <h1 class="tool-title">
                            { "Contact Us" }
                        </h1>
                        <div class="tool-intro">
                            <h2>{ "Get in Touch" }</h2>
                            <p>
                                { "We'd love to hear from you! Whether you have questions, suggestions, feedback, or need support, we're here to help. CompuTools is built for the community, and your input helps us improve and grow." }
                            </p>
                            
                            <h2>{ "How to Reach Us" }</h2>
                            <div style="background: #AD8B73; padding: 30px; border-radius: 8px; margin: 20px 0;">
                                <h3>{ "Email Support" }</h3>
                                <p>
                                    <strong>{ "Primary Contact: " }</strong>
                                    <a href="mailto:tooliverse0520@gmail.com" style="font-size: 1.1em; font-weight: bold;">
                                        { "tooliverse0520@gmail.com" }
                                    </a>
                                </p>
                                <p>
                                    { "We typically respond to emails within 24-48 hours. For urgent issues, please mention \"URGENT\" in your subject line." }
                                </p>
                            </div>
                            
                            <h2>{ "What You Can Contact Us About" }</h2>
                            <ul>
                                <li><strong>{ "Bug Reports" }</strong>{ " - Found something that isn't working correctly? Let us know!" }</li>
                                <li><strong>{ "Feature Requests" }</strong>{ " - Have an idea for a new tool or improvement? We want to hear it!" }</li>
                                <li><strong>{ "Technical Support" }</strong>{ " - Need help using any of our tools? We're here to assist." }</li>
                                <li><strong>{ "General Feedback" }</strong>{ " - Tell us what you love or what could be better." }</li>
                                <li><strong>{ "Business Inquiries" }</strong>{ " - Interested in partnerships or collaborations?" }</li>
                                <li><strong>{ "Educational Use" }</strong>{ " - Questions about using CompuTools in educational settings?" }</li>
                                <li><strong>{ "API Requests" }</strong>{ " - Interested in programmatic access to our tools?" }</li>
                            </ul>
                            
                            <h2>{ "When Contacting Us, Please Include" }</h2>
                            <ul>
                                <li>{ "A clear description of your question or issue" }</li>
                                <li>{ "Which tool you were using (if applicable)" }</li>
                                <li>{ "Your browser and operating system (for technical issues)" }</li>
                                <li>{ "Steps to reproduce the problem (for bug reports)" }</li>
                                <li>{ "Any error messages you received" }</li>
                            </ul>
                            
                            <h2>{ "Response Times" }</h2>
                            <ul>
                                <li><strong>{ "General Inquiries: " }</strong>{ "1-2 business days" }</li>
                                <li><strong>{ "Technical Support: " }</strong>{ "1-3 business days" }</li>
                                <li><strong>{ "Bug Reports: " }</strong>{ "2-5 business days (depending on complexity)" }</li>
                                <li><strong>{ "Feature Requests: " }</strong>{ "We'll acknowledge receipt within 1-2 days and provide timeline estimates" }</li>
                            </ul>
                            
                            <h2>{ "Frequently Asked Questions" }</h2>
                            
                            <h3>{ "Is CompuTools free to use?" }</h3>
                            <p>
                                { "Yes! CompuTools is completely free to use. There are no hidden fees, premium tiers, or usage limits." }
                            </p>
                            
                            <h3>{ "Do you store my data?" }</h3>
                            <p>
                                { "No, we don't store any of the data you input into our tools. All processing happens locally in your browser using WebAssembly technology. Your data never leaves your device." }
                            </p>
                            
                            <h3>{ "Can I use CompuTools offline?" }</h3>
                            <p>
                                { "Once loaded, many of our tools can work offline since they run in your browser. However, you'll need an internet connection to initially access the website." }
                            </p>
                            
                            <h3>{ "Why isn't a tool working in my browser?" }</h3>
                            <p>
                                { "CompuTools requires a modern browser with WebAssembly support. Please ensure you're using an up-to-date version of Chrome, Firefox, Safari, or Edge." }
                            </p>
                            
                            <h3>{ "Can I contribute to CompuTools?" }</h3>
                            <p>
                                { "Absolutely! We welcome contributions, feedback, and suggestions. Contact us to learn about opportunities to contribute." }
                            </p>
                            
                            <h2>{ "Community Guidelines" }</h2>
                            <p>
                                { "When contacting us, please:" }
                            </p>
                            <ul>
                                <li>{ "Be respectful and professional" }</li>
                                <li>{ "Provide clear and detailed information" }</li>
                                <li>{ "Be patient with response times" }</li>
                                <li>{ "Avoid sending duplicate messages unless it's been more than a week" }</li>
                            </ul>
                            
                            <h2>{ "Our Commitment to You" }</h2>
                            <p>
                                { "We're committed to:" }
                            </p>
                            <ul>
                                <li>{ "Responding to all legitimate inquiries" }</li>
                                <li>{ "Continuously improving our tools based on your feedback" }</li>
                                <li>{ "Maintaining the free and open nature of CompuTools" }</li>
                                <li>{ "Protecting your privacy and data" }</li>
                                <li>{ "Building a helpful and supportive community" }</li>
                            </ul>
                            
                            <div style="background: #AD8B73; padding: 20px; border-radius: 8px; margin: 30px 0; border-left: 4px solid #2196f3;">
                                <h3>{ "Quick Contact" }</h3>
                                <p>
                                    { "For the fastest response, email us at " }
                                    <a href="mailto:tooliverse0520@gmail.com">{ "tooliverse0520@gmail.com" }</a>
                                    { " with a clear subject line describing your inquiry." }
                                </p>
                            </div>
                            
                            <p style="margin-top: 40px; font-style: italic; color: #666;">
                                { "Thank you for being part of the CompuTools community. Your feedback and support help us build better tools for everyone!" }
                            </p>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("Contact Us - CompuTools");
                    
                    if let Some(meta_tag) = doc.query_selector("meta[name=\"description\"]").unwrap() {
                        meta_tag.set_attribute("content", "Contact CompuTools for support, feedback, feature requests, or general inquiries. We're here to help with all your computational tool needs.").unwrap();
                    }
                }
            }
        }
    }
} 