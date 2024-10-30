use std::{
	cell::LazyCell,
	fmt::{self, Display},
	str::FromStr,
};

use anyhow::bail;
use chrono::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const C: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"^[0-9]{4}(-[0-9]{2})?(-[0-9]{2})?$").unwrap());

#[doc = "Similar to the standard date type, but each section after the year is optional. e.g. 2014-06-29 or 2023-04"]
#[derive(Clone, Debug)]
pub enum Iso8601 {
	Year(u16),
	YearMonth(u16, u8),
	Full(u16, u8, u8),
}

impl FromStr for Iso8601 {
	type Err = anyhow::Error;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match C.find(value) {
			None => bail!("not a valid date. Date must match one of these formats - YYYY, YYYY-MM, YYYY-MM-DD"),
			Some(_) => {
				let mut parts = value.split('-');

				let year = match parts.next() {
					Some(v) => v.parse()?,
					None => unreachable!(),
				};

				let month = match parts.next() {
					Some(v) => v.parse()?,
					None => return Ok(Iso8601::Year(year)),
				};

				let day: u8 = match parts.next() {
					Some(v) => v.parse()?,
					None => return Ok(Iso8601::YearMonth(year, month)),
				};

				match NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
					Some(_) => Ok(Iso8601::Full(year, month, day)),
					None => bail!("invalid or out-of-range date {}/{}/{}", year, month, day),
				}
			}
		}
	}
}

impl<'de> Deserialize<'de> for Iso8601 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.parse()
			.map_err(|e: anyhow::Error| <D::Error as serde::de::Error>::custom(e.to_string()))
	}
}

impl Serialize for Iso8601 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let ser_val = match self {
			Iso8601::Year(y) => format!("{}", y),
			Iso8601::YearMonth(y, m) => format!("{}-{}", y, m),
			Iso8601::Full(y, m, d) => format!("{}-{}-{}", y, m, d),
		};

		serializer.serialize_str(&ser_val)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Resume {
	#[doc = "Specify any awards you have received throughout your professional career"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub awards: Vec<AwardsItem>,
	#[serde(default)]
	pub basics: Basics,
	#[doc = "Specify any certificates you have received throughout your professional career"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub certificates: Vec<CertificatesItem>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub education: Vec<EducationItem>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub interests: Vec<InterestsItem>,
	#[doc = "List any other languages you speak"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub languages: Vec<LanguagesItem>,
	#[serde(default)]
	pub meta: Meta,
	#[doc = "Specify career projects"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub projects: Vec<ProjectsItem>,
	#[doc = "Specify your publications through your career"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub publications: Vec<PublicationsItem>,
	#[doc = "List references you have received"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub references: Vec<ReferencesItem>,
	#[doc = "link to the version of the schema that can validate the resume"]
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
	pub schema: Option<String>,
	#[doc = "List out your professional skill-set"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub skills: Vec<SkillsItem>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub volunteer: Vec<VolunteerItem>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub work: Vec<WorkItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AwardsItem {
	#[doc = "e.g. Time Magazine"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub awarder: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date: Option<Iso8601>,
	#[doc = "e.g. Received for my work with Quantum Physics"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	#[doc = "e.g. One of the 100 greatest minds of the century"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Basics {
	#[doc = "e.g. thomas@gmail.com"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	#[doc = "URL (as per RFC 3986) to a image in JPEG or PNG format"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub image: Option<String>,
	#[doc = "e.g. Web Developer"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(default)]
	pub location: BasicsLocation,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,
	#[doc = "Specify any number of social networks that you participate in"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub profiles: Vec<BasicsProfilesItem>,
	#[doc = "Write a short 2-3 sentence biography about yourself"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	#[doc = "URL (as per RFC 3986) to your website, e.g. personal homepage"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BasicsLocation {
	#[doc = "To add multiple address lines, use \n. For example, 1234 Glücklichkeit Straße\nHinterhaus 5. Etage li."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	#[doc = "code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN"]
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "countryCode")]
	pub country_code: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "postalCode")]
	pub postal_code: Option<String>,
	#[doc = "The general region where you live. Can be a US state, or a province, for instance."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BasicsProfilesItem {
	#[doc = "e.g. Facebook or Twitter"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub network: Option<String>,
	#[doc = "e.g. http://twitter.example.com/neutralthoughts"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[doc = "e.g. neutralthoughts"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificatesItem {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date: Option<Iso8601>,
	#[doc = "e.g. CNCF"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issuer: Option<String>,
	#[doc = "e.g. Certified Kubernetes Administrator"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "e.g. http://example.com"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EducationItem {
	#[doc = "e.g. Arts"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub area: Option<String>,
	#[doc = "List notable courses/subjects"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub courses: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "endDate")]
	pub end_date: Option<Iso8601>,
	#[doc = "e.g. Massachusetts Institute of Technology"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub institution: Option<String>,
	#[doc = "grade point average, e.g. 3.67/4.0"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub score: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "startDate")]
	pub start_date: Option<Iso8601>,
	#[doc = "e.g. Bachelor"]
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "studyType")]
	pub study_type: Option<String>,
	#[doc = "e.g. http://facebook.example.com"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct InterestsItem {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<String>,
	#[doc = "e.g. Philosophy"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LanguagesItem {
	#[doc = "e.g. Fluent, Beginner"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub fluency: Option<String>,
	#[doc = "e.g. English, Spanish"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
}

#[doc = "The schema version and any other tooling configuration lives here"]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Meta {
	#[doc = "URL (as per RFC 3986) to latest version of this document"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub canonical: Option<String>,
	#[doc = "Using ISO 8601 with YYYY-MM-DDThh:mm:ss"]
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModified")]
	pub last_modified: Option<String>,
	#[doc = "A version field which follows semver - e.g. v1.0.0"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProjectsItem {
	#[doc = "Short summary of project. e.g. Collated works of 2017."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "endDate")]
	pub end_date: Option<Iso8601>,
	#[doc = "Specify the relevant company/entity affiliations e.g. 'greenpeace', 'corporationXYZ'"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	#[doc = "Specify multiple features"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub highlights: Vec<String>,
	#[doc = "Specify special elements involved"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<String>,
	#[doc = "e.g. The World Wide Web"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "Specify your role on this project or in company"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub roles: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "startDate")]
	pub start_date: Option<Iso8601>,
	#[doc = " e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'"]
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
	pub project_type: Option<String>,
	#[doc = "e.g. http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PublicationsItem {
	#[doc = "e.g. The World Wide Web"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "e.g. IEEE, Computer Magazine"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub publisher: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "releaseDate")]
	pub release_date: Option<Iso8601>,
	#[doc = "Short summary of publication. e.g. Discussion of the World Wide Web, HTTP, HTML."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	#[doc = "e.g. http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ReferencesItem {
	#[doc = "e.g. Timothy Cook"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "e.g. Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub reference: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SkillsItem {
	#[doc = "List some keywords pertaining to this skill"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<String>,
	#[doc = "e.g. Master"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub level: Option<String>,
	#[doc = "e.g. Web Development"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct VolunteerItem {
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "endDate")]
	pub end_date: Option<Iso8601>,
	#[doc = "Specify accomplishments and achievements"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub highlights: Vec<String>,
	#[doc = "e.g. Facebook"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub organization: Option<String>,
	#[doc = "e.g. Software Engineer"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "startDate")]
	pub start_date: Option<Iso8601>,
	#[doc = "Give an overview of your responsibilities at the company"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	#[doc = "e.g. http://facebook.example.com"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WorkItem {
	#[doc = "e.g. Social Media Company"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "endDate")]
	pub end_date: Option<Iso8601>,
	#[doc = "Specify multiple accomplishments"]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub highlights: Vec<String>,
	#[doc = "e.g. Menlo Park, CA"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	#[doc = "e.g. Facebook"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[doc = "e.g. Software Engineer"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "startDate")]
	pub start_date: Option<Iso8601>,
	#[doc = "Give an overview of your responsibilities at the company"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	#[doc = "e.g. http://facebook.example.com"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

macro_rules! impl_display_from_ser {
	($($Type:ty)*) => {
		$(
		impl Display for $Type {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				let ser = json5::to_string(self).map_err(|_| fmt::Error)?;
				write!(f, "{}", ser)
			}
		})*
	};
}

impl_display_from_ser! {
	Iso8601 Resume AwardsItem Basics BasicsLocation
	BasicsProfilesItem CertificatesItem EducationItem
	InterestsItem LanguagesItem Meta ProjectsItem
	PublicationsItem ReferencesItem SkillsItem
	VolunteerItem WorkItem
}
