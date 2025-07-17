use yew::prelude::*;
use web_sys::window;

pub struct Privacy {}

impl Component for Privacy {
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
                            { "Privacy Policy" }
                        </h1>
                        <div class="tool-intro">
                            <p><strong>{ "Last updated: " }</strong>{ "January 2025" }</p>
                            
                            <h2>{ "Introduction" }</h2>
                            <p>
                                { "CompuTools (\"we,\" \"our,\" or \"us\") is committed to protecting your privacy. This Privacy Policy explains how we collect, use, and safeguard your information when you use our website and tools (the \"Service\")." }
                            </p>
                            
                            <h2>{ "Information We Collect" }</h2>
                            
                            <h3>{ "Information You Provide" }</h3>
                            <p>
                                { "We only collect information that you voluntarily provide to us, such as:" }
                            </p>
                            <ul>
                                <li>{ "Email addresses when you contact us for support or feedback" }</li>
                                <li>{ "Any information you choose to include in your communications with us" }</li>
                            </ul>
                            
                            <h3>{ "Information Automatically Collected" }</h3>
                            <p>
                                { "When you visit our website, we may automatically collect certain technical information, including:" }
                            </p>
                            <ul>
                                <li>{ "Browser type and version" }</li>
                                <li>{ "Operating system" }</li>
                                <li>{ "IP address (anonymized)" }</li>
                                <li>{ "Pages visited and time spent on our website" }</li>
                                <li>{ "Referring website" }</li>
                            </ul>
                            
                            <h2>{ "Data Processing and Storage" }</h2>
                            
                            <h3>{ "Local Processing Only" }</h3>
                            <p>
                                { "All computational tasks and data conversions performed by our tools happen entirely within your browser using WebAssembly technology. We do not:" }
                            </p>
                            <ul>
                                <li>{ "Store any data you input into our tools" }</li>
                                <li>{ "Transmit your data to our servers" }</li>
                                <li>{ "Keep records of your calculations or conversions" }</li>
                                <li>{ "Access files you process through our tools" }</li>
                            </ul>
                            
                            <h3>{ "Local Storage" }</h3>
                            <p>
                                { "Our website may use local storage in your browser to:" }
                            </p>
                            <ul>
                                <li>{ "Remember your theme preferences (light/dark mode)" }</li>
                                <li>{ "Store recently used tools for convenience" }</li>
                                <li>{ "Maintain your settings across sessions" }</li>
                            </ul>
                            <p>
                                { "This information is stored locally on your device and is not transmitted to us." }
                            </p>
                            
                            <h2>{ "How We Use Information" }</h2>
                            <p>
                                { "We use the limited information we collect to:" }
                            </p>
                            <ul>
                                <li>{ "Provide and maintain our Service" }</li>
                                <li>{ "Respond to your questions and support requests" }</li>
                                <li>{ "Improve our website and tools" }</li>
                                <li>{ "Analyze website usage patterns (in aggregate, anonymized form)" }</li>
                                <li>{ "Comply with legal obligations" }</li>
                            </ul>
                            
                            <h2>{ "Third-Party Services" }</h2>
                            
                            <h3>{ "Analytics" }</h3>
                            <p>
                                { "We use Google Analytics to understand how visitors interact with our website. Google Analytics collects information anonymously and reports website usage statistics. You can opt out of Google Analytics by installing the Google Analytics opt-out browser add-on." }
                            </p>
                            
                            <h3>{ "Advertising" }</h3>
                            <p>
                                { "We may display advertisements through Google AdSense. Google may use cookies to serve ads based on your prior visits to our website or other websites. You can opt out of personalized advertising by visiting Google's Ad Settings." }
                            </p>
                            
                            <h2>{ "Cookies and Similar Technologies" }</h2>
                            <p>
                                { "We use cookies and similar technologies to:" }
                            </p>
                            <ul>
                                <li>{ "Remember your preferences" }</li>
                                <li>{ "Analyze site usage" }</li>
                                <li>{ "Provide relevant advertisements" }</li>
                            </ul>
                            <p>
                                { "You can control cookies through your browser settings. However, disabling cookies may affect the functionality of our website." }
                            </p>
                            
                            <h2>{ "Data Security" }</h2>
                            <p>
                                { "We implement appropriate security measures to protect your information. Since most data processing happens locally in your browser, your sensitive information never leaves your device. However, no method of transmission over the internet is 100% secure." }
                            </p>
                            
                            <h2>{ "Your Rights" }</h2>
                            <p>
                                { "Depending on your location, you may have certain rights regarding your personal information, including:" }
                            </p>
                            <ul>
                                <li>{ "Right to access your personal information" }</li>
                                <li>{ "Right to correct inaccurate information" }</li>
                                <li>{ "Right to delete your personal information" }</li>
                                <li>{ "Right to restrict processing" }</li>
                                <li>{ "Right to data portability" }</li>
                            </ul>
                            
                            <h2>{ "Children's Privacy" }</h2>
                            <p>
                                { "Our Service is not intended for children under 13 years of age. We do not knowingly collect personal information from children under 13. If you are a parent or guardian and believe your child has provided us with personal information, please contact us." }
                            </p>
                            
                            <h2>{ "International Data Transfers" }</h2>
                            <p>
                                { "Since our tools process data locally in your browser, there are typically no international data transfers. Any limited data we do collect may be processed in countries other than your own, and we ensure appropriate safeguards are in place." }
                            </p>
                            
                            <h2>{ "Changes to This Privacy Policy" }</h2>
                            <p>
                                { "We may update this Privacy Policy from time to time. We will notify users of any material changes by posting the new Privacy Policy on this page and updating the \"Last updated\" date." }
                            </p>
                            
                            <h2>{ "Contact Information" }</h2>
                            <p>
                                { "If you have any questions about this Privacy Policy or our privacy practices, please contact us at:" }
                            </p>
                            <p>
                                <strong>{ "Email: " }</strong>
                                <a href="mailto:tooliverse0520@gmail.com">{ "tooliverse0520@gmail.com" }</a>
                            </p>
                            
                            <p style="margin-top: 40px; font-style: italic; color: #666;">
                                { "We are committed to transparency and protecting your privacy. This policy reflects our privacy-first approach to building useful tools." }
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
                    doc.set_title("Privacy Policy - CompuTools");
                    
                    if let Some(meta_tag) = doc.query_selector("meta[name=\"description\"]").unwrap() {
                        meta_tag.set_attribute("content", "CompuTools Privacy Policy - Learn how we protect your privacy and handle data. All processing happens locally in your browser with no data transmission.").unwrap();
                    }
                }
            }
        }
    }
} 