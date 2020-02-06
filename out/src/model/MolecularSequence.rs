#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::MolecularSequence_Quality::MolecularSequence_Quality;
use crate::model::MolecularSequence_ReferenceSeq::MolecularSequence_ReferenceSeq;
use crate::model::MolecularSequence_Variant::MolecularSequence_Variant;
use crate::model::Quantity::Quantity;
use crate::model::MolecularSequence_StructureVariant::MolecularSequence_StructureVariant;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::MolecularSequence_Repository::MolecularSequence_Repository;


/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence {
  /// The base language in which the resource is written.
  language: Option<String>,

  /// Whether the sequence is numbered starting at 0 (0-based numbering or
  /// coordinates, inclusive start, exclusive end) or starting at 1 (1-based
  /// numbering, inclusive start and inclusive end).
  #[serde(rename = "coordinateSystem")]
  coordinate_system: Option<i32>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Amino Acid Sequence/ DNA Sequence / RNA Sequence.
  #[serde(rename = "type")]
  fhir_type: Option<MolecularSequenceType>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The method for sequencing, for example, chip information.
  device: Option<Box<Reference>>,

  /// The number of copies of the sequence of interest. (RNASeq).
  quantity: Option<Quantity>,

  /// Extensions for readCoverage
  #[serde(rename = "_readCoverage")]
  _read_coverage: Option<Element>,

  /// The definition of variant here originates from Sequence ontology
  /// ([variant_of](http://www.sequenceontology.org/browser/current_svn/term/variant_o
  /// f)). This element can represent amino acid or nucleic sequence change(including
  /// insertion,deletion,SNP,etc.)  It can represent some complex mutation or segment
  /// variation with the assist of CIGAR string.
  variant: Option<Vec<MolecularSequence_Variant>>,

  /// Information about chromosome structure variation.
  #[serde(rename = "structureVariant")]
  structure_variant: Option<Vec<MolecularSequence_StructureVariant>>,

  /// A unique identifier for this particular sequence instance. This is a FHIR-
  /// defined id.
  identifier: Option<Vec<Identifier>>,

  /// A sequence that is used as a reference to describe variants that are present in
  /// a sequence analyzed.
  #[serde(rename = "referenceSeq")]
  reference_seq: Option<MolecularSequence_ReferenceSeq>,

  /// The organization or lab that should be responsible for this result.
  performer: Option<Box<Reference>>,

  /// Specimen used for sequencing.
  specimen: Option<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for observedSeq
  #[serde(rename = "_observedSeq")]
  _observed_seq: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Configurations of the external repository. The repository shall store target's
  /// observedSeq or records related with target's observedSeq.
  repository: Option<Vec<MolecularSequence_Repository>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The patient whose sequencing results are described by this resource.
  patient: Option<Box<Reference>>,

  /// An experimental feature attribute that defines the quality of the feature in a
  /// quantitative way, such as a phred quality score
  /// ([SO:0001686](http://www.sequenceontology.org/browser/current_svn/term/SO:000168
  /// 6)).
  quality: Option<Vec<MolecularSequence_Quality>>,

  /// Sequence that was observed. It is the result marked by referenceSeq along with
  /// variant records on referenceSeq. This shall start from referenceSeq.windowStart
  /// and end by referenceSeq.windowEnd.
  #[serde(rename = "observedSeq")]
  observed_seq: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for coordinateSystem
  #[serde(rename = "_coordinateSystem")]
  _coordinate_system: Option<Element>,

  /// Pointer to next atomic sequence which at most contains one variant.
  pointer: Option<Vec<Box<Reference>>>,

  /// Coverage (read depth or depth) is the average number of reads representing a
  /// given nucleotide in the reconstructed sequence.
  #[serde(rename = "readCoverage")]
  read_coverage: Option<i32>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MolecularSequenceType {
  #[serde(rename = "aa")]
  Aa,

  #[serde(rename = "dna")]
  Dna,

  #[serde(rename = "rna")]
  Rna,

}