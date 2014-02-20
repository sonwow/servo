/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::DocumentTypeBinding;
use dom::document::AbstractDocument;
use dom::node::{AbstractNode, Node, DoctypeNodeTypeId};
use servo_util::str::DOMString;

use std::mem::size_of_val;

/// The `DOCTYPE` tag.
pub struct DocumentType {
    node: Node,
    name: DOMString,
    public_id: DOMString,
    system_id: DOMString,
}

impl DocumentType {
    pub fn new_inherited(name: DOMString,
                         public_id: Option<DOMString>,
                         system_id: Option<DOMString>,
                         document: AbstractDocument)
            -> DocumentType {
        let docType = DocumentType {
            node: Node::new_inherited(DoctypeNodeTypeId, document),
            name: name,
            public_id: public_id.unwrap_or(~""),
            system_id: system_id.unwrap_or(~"")
        };
        println!("[DOM] DocumentType: {:?}", size_of_val(&docType));
        docType
    }

    pub fn new(name: DOMString,
               public_id: Option<DOMString>,
               system_id: Option<DOMString>,
               document: AbstractDocument)
               -> AbstractNode {
        let documenttype = DocumentType::new_inherited(name,
                                                       public_id,
                                                       system_id,
                                                       document);
        Node::reflect_node(@mut documenttype, document, DocumentTypeBinding::Wrap)
    }
}

impl DocumentType {
    pub fn Name(&self) -> DOMString {
        self.name.clone()
    }

    pub fn PublicId(&self) -> DOMString {
        self.public_id.clone()
    }

    pub fn SystemId(&self) -> DOMString {
        self.system_id.clone()
    }
}
