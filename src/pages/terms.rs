use yew::prelude::*;
use web_sys::window;

pub struct Terms {}

impl Component for Terms {
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
                            { "Terms of Service" }
                        </h1>
                        <div class="tool-intro">
                            <p><strong>{ "Last updated: " }</strong>{ "January 2025" }</p>
                            
                            <h2>{ "Agreement to Terms" }</h2>
                            <p>
                                { "By accessing and using CompuTools (\"Service\"), you accept and agree to be bound by the terms and provision of this agreement. If you do not agree to abide by the above, please do not use this service." }
                            </p>
                            
                            <h2>{ "Description of Service" }</h2>
                            <p>
                                { "CompuTools is a web-based platform that provides computational and conversion tools for engineering, development, and educational purposes. Our tools include but are not limited to:" }
                            </p>
                            <ul>
                                <li>{ "Number base conversions" }</li>
                                <li>{ "Text encoding and decoding utilities" }</li>
                                <li>{ "Hash generators and calculators" }</li>
                                <li>{ "Time and date conversion tools" }</li>
                                <li>{ "Data format converters" }</li>
                                <li>{ "Mathematical calculation utilities" }</li>
                            </ul>
                            
                            <h2>{ "Acceptable Use" }</h2>
                            <p>
                                { "You may use our Service for lawful purposes only. You agree not to use the Service:" }
                            </p>
                            <ul>
                                <li>{ "In any way that violates any applicable federal, state, local, or international law or regulation" }</li>
                                <li>{ "To transmit, or procure the sending of, any advertising or promotional material, or any other form of similar solicitation (spam)" }</li>
                                <li>{ "To impersonate or attempt to impersonate the Company, a Company employee, another user, or any other person or entity" }</li>
                                <li>{ "To engage in any other conduct that restricts or inhibits anyone's use or enjoyment of the Service" }</li>
                                <li>{ "For any purpose that is unlawful or prohibited by these Terms" }</li>
                            </ul>
                            
                            <h2>{ "Intellectual Property Rights" }</h2>
                            <p>
                                { "The Service and its original content, features, and functionality are and will remain the exclusive property of CompuTools and its licensors. The Service is protected by copyright, trademark, and other laws. Our trademarks and trade dress may not be used in connection with any product or service without our prior written consent." }
                            </p>
                            
                            <h2>{ "User Data and Privacy" }</h2>
                            <p>
                                { "We are committed to protecting your privacy. All data processing happens locally in your browser using WebAssembly technology. We do not store, transmit, or have access to the data you input into our tools. For more information, please review our Privacy Policy." }
                            </p>
                            
                            <h2>{ "Disclaimer of Warranties" }</h2>
                            <p>
                                { "The information on this Service is provided on an \"as is\" basis. To the fullest extent permitted by law, this Company:" }
                            </p>
                            <ul>
                                <li>{ "Excludes all representations and warranties relating to this Service and its contents" }</li>
                                <li>{ "Excludes all liability for damages arising out of or in connection with your use of this Service" }</li>
                            </ul>
                            <p>
                                { "While we strive to provide accurate and reliable tools, we cannot guarantee that our calculations and conversions are error-free. Users should verify important results independently." }
                            </p>
                            
                            <h2>{ "Limitation of Liability" }</h2>
                            <p>
                                { "In no event shall CompuTools, nor its directors, employees, partners, agents, suppliers, or affiliates, be liable for any indirect, incidental, special, consequential, or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses, resulting from your use of the Service." }
                            </p>
                            
                            <h2>{ "Availability and Technical Requirements" }</h2>
                            <p>
                                { "Our Service requires a modern web browser with WebAssembly support. While we strive to maintain 99.9% uptime, we cannot guarantee uninterrupted access to the Service. We reserve the right to:" }
                            </p>
                            <ul>
                                <li>{ "Modify or discontinue the Service (or any part thereof) temporarily or permanently" }</li>
                                <li>{ "Refuse service to anyone for any reason at any time" }</li>
                                <li>{ "Update, improve, or add new features to the Service" }</li>
                            </ul>
                            
                            <h2>{ "Third-Party Services" }</h2>
                            <p>
                                { "Our Service may contain links to third-party web sites or services that are not owned or controlled by CompuTools. We have no control over, and assume no responsibility for, the content, privacy policies, or practices of any third-party websites or services." }
                            </p>
                            
                            <h2>{ "Indemnification" }</h2>
                            <p>
                                { "You agree to defend, indemnify, and hold harmless CompuTools and its licensee and licensors, and their employees, contractors, agents, officers and directors, from and against any and all claims, damages, obligations, losses, liabilities, costs or debt, and expenses (including but not limited to attorney's fees)." }
                            </p>
                            
                            <h2>{ "Governing Law" }</h2>
                            <p>
                                { "These Terms shall be interpreted and governed by the laws of the jurisdiction in which CompuTools operates, without regard to its conflict of law provisions. Our failure to enforce any right or provision of these Terms will not be considered a waiver of those rights." }
                            </p>
                            
                            <h2>{ "Changes to Terms" }</h2>
                            <p>
                                { "We reserve the right, at our sole discretion, to modify or replace these Terms at any time. If a revision is material, we will try to provide at least 30 days notice prior to any new terms taking effect." }
                            </p>
                            
                            <h2>{ "Termination" }</h2>
                            <p>
                                { "We may terminate or suspend your access immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms. Upon termination, your right to use the Service will cease immediately." }
                            </p>
                            
                            <h2>{ "Severability" }</h2>
                            <p>
                                { "If any provision of these Terms is held to be unenforceable or invalid, such provision will be changed and interpreted to accomplish the objectives of such provision to the greatest extent possible under applicable law and the remaining provisions will continue in full force and effect." }
                            </p>
                            
                            <h2>{ "Contact Information" }</h2>
                            <p>
                                { "If you have any questions about these Terms of Service, please contact us at:" }
                            </p>
                            <p>
                                <strong>{ "Email: " }</strong>
                                <a href="mailto:tooliverse0520@gmail.com">{ "tooliverse0520@gmail.com" }</a>
                            </p>
                            
                            <p style="margin-top: 40px; font-style: italic; color: #666;">
                                { "By using CompuTools, you acknowledge that you have read and understood these terms and agree to be bound by them." }
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
                    doc.set_title("Terms of Service - CompuTools");
                    
                    if let Some(meta_tag) = doc.query_selector("meta[name=\"description\"]").unwrap() {
                        meta_tag.set_attribute("content", "CompuTools Terms of Service - Read our terms and conditions for using our computational and conversion tools platform.").unwrap();
                    }
                }
            }
        }
    }
} 