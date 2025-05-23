import os
import pickle
from typing import Optional
from pypdf import PdfReader
import requests
import json


class Question:
    def __init__(self, question: str, answers: list[str], correct_answer: str):
        self.question = question
        self.answers = answers
        self.correct_answer = correct_answer

class ChemrxivData:
    def __init__(self, data):
        self.data = data
        self.viewer = data.get('viewer', {})
        self.usage_events_disabled = self.viewer.get('usageEventsDisabled', False)
        self.user = self.viewer.get('user')
        self.search_items = self.viewer.get('searchItems', {})
        self.total_count = self.search_items.get('totalCount', 0)
        self.results = self.search_items.get('results', [])
        self.items = [ChemrxivItem(item_data['item']) for item_data in self.results]


class ChemrxivItem:
    def __init__(self, item_data):
        self.item_data = item_data.get('item')
        self.typename = self.item_data.get('__typename', '')
        self.id = self.item_data.get('id', '')
        self.title = self.item_data.get('title', '')
        self.abstract = self.item_data.get('abstract', '')
        self.keywords = self.item_data.get('keywords', [])
        self.origin = self.item_data.get('origin', '')
        self.version = self.item_data.get('version', '')
        self.published_date = self.item_data.get('publishedDate', '')
        self.submitted_date = self.item_data.get('submittedDate', '')
        self.subject_type = Subject(self.item_data.get('subjectType', {}))
        self.content_type = ContentType(self.item_data.get('contentType', {}))
        self.category_types = [Category(cat) for cat in self.item_data.get('categoryTypes', [])]
        self.main_category = self.item_data.get('mainCategory', {}).get('name', '')
        self.asset = Asset(self.item_data.get('asset', {}))
        self.authors = [Author(author) for author in self.item_data.get('authors', [])]
        self.metrics = [Metric(metric) for metric in self.item_data.get('metrics', [])]
        self.citations_count = self.item_data.get('citationsCount', 0)
        self.community = self.item_data.get('community', '')
    def to_frontend_article(self, questions: list[Question]):
        return FrontendArticle(self, questions)
    def __str__(self):
        return (
            f"Title: {self.title}\n"
            f"Abstract: {self.abstract}\n"
            f"Keywords: {', '.join(self.keywords)}\n"
            f"Published Date: {self.published_date}\n"
        )
    def to_pickle(self):
        return pickle.dumps(self)
    def get_pdf_with_text(self) -> Optional[str]:
        if os.path.exists(f"{self.title}.pdf"):
            pass
        else:
            url = self.asset.original.url
            response = requests.get(url)
            with open(f"{self.title}.pdf", "wb") as f:
                f.write(response.content)
        try:
            with open(f"{self.title}.pdf", "rb") as f:
                pdf_reader = PdfReader(f)
                paper = ""
                for page in pdf_reader.pages:
                    paper += page.extract_text()
                return paper
        except FileNotFoundError:
            return None
        


class Subject:
    def __init__(self, subject_data):
        self.subject_data = subject_data
        self.typename = self.subject_data.get('__typename', '')
        self.id = self.subject_data.get('id', '')
        self.name = self.subject_data.get('name', '')
        self.description = self.subject_data.get('description', '')


class ContentType:
    def __init__(self, content_data):
        self.content_data = content_data
        self.typename = self.content_data.get('__typename', '')
        self.id = self.content_data.get('id', '')
        self.name = self.content_data.get('name', '')
        self.allow_submission = self.content_data.get('allowSubmission', False)
        self.allow_journal_submission = self.content_data.get('allowJournalSubmission', False)
        self.allow_community_submission = self.content_data.get('allowCommunitySubmission', False)
        self.allow_research_direction_submission = self.content_data.get('allowResearchDirectionSubmission', False)
        self.video_allowed_check = self.content_data.get('videoAllowedCheck', False)
        self.allowed_file_types = self.content_data.get('allowedFileTypes', [])
        self.allowed_video_file_types = self.content_data.get('allowedVideoFileTypes', [])


class Category:
    def __init__(self, category_data):
        self.category_data = category_data
        self.typename = self.category_data.get('__typename', '')
        self.id = self.category_data.get('id', '')
        self.name = self.category_data.get('name', '')
        self.description = self.category_data.get('description', '')
        self.parent_id = self.category_data.get('parentId', '')


class Asset:
    def __init__(self, asset_data):
        self.asset_data = asset_data
        self.mime_type = self.asset_data.get('mimeType', '')
        self.original = Original(self.asset_data.get('original', {}))


class Original:
    def __init__(self, original_data):
        self.original_data = original_data
        self.url = self.original_data.get('url', '')


class Author:
    def __init__(self, author_data):
        self.author_data = author_data
        self.title = self.author_data.get('title', '')
        self.first_name = self.author_data.get('firstName', '')
        self.last_name = self.author_data.get('lastName', '')
        self.author_confirmation_id = self.author_data.get('authorConfirmationId', '')
        self.display_order = self.author_data.get('displayOrder', 0)


class Metric:
    def __init__(self, metric_data):
        self.metric_data = metric_data
        self.metric_type = self.metric_data.get('metricType', '')
        self.description = self.metric_data.get('description', '')
        self.value = self.metric_data.get('value', 0)
        self.unit = self.metric_data.get('unit', '')

from typing import List, Dict, Any, Optional


class FrontendArticle:
    def __init__(self, item: ChemrxivItem, questions: list[Question]):
        self.title = item.title
        self.description = item.abstract
        self.body = item.abstract
        self.image = FrontendImage({'url': "https://placecat.com/640/360", 'alt': "A placeholder image of a cat", 'caption': "A cute cat"})
        self.authors = [FrontendAuthor(author) for author in item.authors]
        self.questions = questions
        self.professor = FrontendProfessor({'name': "Dr. Fake Name", 'professorBio': "This is a fake professor bio.", 'slug': "fake-professor"})
        self.createdAt = item.published_date
        self.publishedAt = item.published_date
        self.updatedAt = None
        self.lastUpdatedAt = None
    def export_to_toml(self, file_name: str):
        with open(f"{file_name}.toml", "w") as f:
            f.write(self.generate_article_toml())
    def generate_article_toml(self):
        article_data = f"""
title = "{self.title}"
description = "{self.description}"
body = "{self.body}"
createdAt = "{self.createdAt}"
publishedAt = "{self.publishedAt}"
updatedAt = "{self.updatedAt}"
lastUpdatedAt = "{self.lastUpdatedAt}"
readingTime = 10

[image]
url = "{self.image.url}"
alt = "{self.image.alt}"
caption = "{self.image.caption}"

"""
        for author in self.authors:
            article_data += f"""
[[authors]]
name = "{author.name}"
authorBio = "{author.authorBio}"
slug = "{author.slug}"
"""
        article_data += f"""
[professor]
name = "{self.professor.name}"
professorBio = "{self.professor.professorBio}"
slug = "{self.professor.slug}"
"""
        self.article_data = article_data
        return article_data
class FrontendImage:
    def __init__(self, image_data: Dict[str, Any]):
        self.image_data: Dict[str, Any] = image_data
        self.url: str = self.image_data.get('url', '')
        self.alt: str = self.image_data.get('alt', '')
        self.caption: str = self.image_data.get('caption', '')

class FrontendAuthor:
    def __init__(self, author_data: Author):
        self.name: str = author_data.first_name + " " + author_data.last_name
        self.authorBio: str = "This is a fake author bio."
        self.slug: str = author_data.author_confirmation_id

class FrontendProfessor:
    def __init__(self, professor_data: Dict[str, Any]):
        self.name: str = professor_data['name']
        self.professorBio: str = professor_data['professorBio']
        self.slug: str = professor_data['slug']



def get_items(number_of_items):
    items = []
    skip = 0
    for _ in range(number_of_items // 10):
        headers = {
            'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0',
        'accept': 'application/json, text/plain, */*',
        'accept-language': 'en-US,en;q=0.5',
        'content-type': 'application/json',
        'x-api-key': 'y6nWHrymZysXc',
        'origin': 'https://chemrxiv.org',
        'dnt': '1',
        'sec-gpc': '1',
        'cookie': 'orp_chemrxiv_sess=s%3AVuliL0DGfvY5R87JiORZ4snQd7r3auDo.wHU3FZ4HkWjvOwrAuHw%2FXXSLRUMXbBUOCYXZEGREQJk',
        'sec-fetch-dest': 'empty',
        'sec-fetch-mode': 'cors',
        'sec-fetch-site': 'same-origin',
        'te': 'trailers',
        'referer': 'https://chemrxiv.org/engage/chemrxiv/search-dashboard?publishedDates=LAST_MONTH&categories=605c72ef153207001f6470d3&sortBy=RELEVANT_DESC'
        }

        data = {
            "query": "query searchDashboardPageLoad(\n  $text: String = \"\"\n  $subjects: [String!]\n  $categories: [String!]\n  $events: [String!]\n  $publishedDates: [String!]\n  $partners: [String!]\n  $contents: [String!]\n  $keywords: [String!]\n  $authors: String = \"\"\n  $skip: Int = 0\n  $limit: Int = 10\n  $sortBy: SortByEnum = RELEVANT_DESC\n) {\n  viewer {\n    usageEventsDisabled\n\n    user {\n      ...userRoleFragment\n    }\n\n    searchItems(\n      searchTerm: $text\n      subjectKeys: $subjects\n      categoryKeys: $categories\n      eventKeys: $events\n      publishedDateKeys: $publishedDates\n      partnerKeys: $partners\n      contentTypeKeys: $contents\n      keywordsKeys: $keywords\n      searchAuthor: $authors\n      skip: $skip\n      limit: $limit\n      sortBy: $sortBy\n      includeBuckets: true\n    ) {\n      totalCount\n\n      results: itemHits {\n        highlight {\n          text\n          matchPositions {\n            start\n            end\n          }\n        }\n\n        item {\n          ...itemMatchFragment\n        }\n      }\n\n      subjectBuckets {\n        ...searchBucketFragment\n      }\n\n      categoryBuckets {\n        ...searchBucketFragment\n      }\n\n      eventBuckets {\n        ...searchBucketFragment\n      }\n\n      partnerBuckets {\n        ...searchBucketFragment\n      }\n\n      publishedDateBuckets {\n        ...searchBucketFragment\n      }\n\n      contentBuckets: contentTypeBuckets {\n        ...searchBucketFragment\n      }\n\n      dateBuckets: publishedDateBuckets {\n        ...searchBucketFragment\n      }\n    }\n\n    subjectTypes: subjects {\n      ...subjectTypeFragment\n    }\n\n    contentTypes {\n      ...contentTypeFragment\n    }\n\n    categoryTypes: categories {\n      ...categoryTypeFragment\n    }\n  }\n}\n\nfragment userRoleFragment on User {\n  __typename\n  id\n  sessionExpiresAt\n  titleTypeId: title\n  firstName\n  lastName\n  emailAddress: email\n  orcid\n  roles\n  accountType\n}\n\nfragment itemMatchFragment on MainItem {\n  __typename\n  id\n  title\n  abstract\n  keywords\n  origin\n  version\n  publishedDate\n  submittedDate\n  subjectType: subject {\n    ...subjectTypeFragment\n  }\n  contentType {\n    ...contentTypeFragment\n  }\n  categoryTypes: categories {\n    ...categoryTypeFragment\n  }\n  mainCategory {\n    name\n  }\n  asset {\n    mimeType\n    original {\n      url\n    }\n  }\n  authors {\n    title\n    firstName\n    lastName\n    authorConfirmationId\n    displayOrder\n  }\n  metrics {\n    metricType\n    description\n    value\n    unit\n  }\n  citationsCount\n  community {\n    id\n    name\n  }\n}\n\nfragment searchBucketFragment on SearchBucket {\n  __typename\n  count\n  key\n  label\n}\n\nfragment subjectTypeFragment on Subject {\n  __typename\n  id\n  name\n  description\n}\n\nfragment contentTypeFragment on ContentType {\n  __typename\n  id\n  name\n  allowSubmission\n  allowJournalSubmission\n  allowCommunitySubmission\n  allowResearchDirectionSubmission\n  videoAllowedCheck\n  allowedFileTypes\n  allowedVideoFileTypes\n}\n\nfragment categoryTypeFragment on Category {\n  __typename\n  id\n  name\n  description\n  parentId\n}\n",
            "variables": {
                "sortBy": "RELEVANT_DESC",
                "skip": skip,
                "categories": ["605c72ef153207001f6470d3"],
                "contents": [],
                "events": [],
                "publishedDates": ["LAST_MONTH"],
                "subjects": [],
                "partners": [],
                "keywords": []
            }
        }
        url = "https://chemrxiv.org/engage/api-gateway/chemrxiv/graphql"
        response = requests.post(url, headers=headers, data=json.dumps(data))
        clean_response = response.json()['data']['viewer']['searchItems']['results']
        if clean_response == []:
            break
        items.extend(clean_response)
        skip += 10
        
    return [ChemrxivItem(item) for item in items]

def get_all_items():
    return sorted(get_items(1000), key=lambda x: x.metrics[0].value, reverse=True)


def print_item(item: ChemrxivItem):
    x = json.dumps(
        {
        "title": item.title if item.title else "No Title",
        "url": item.asset.original.url if item.asset and item.asset.original and item.asset.original.url else "No URL",
        "keywords": item.keywords if item.keywords else "No Keywords",
        }
    )
    print(x)


def get_top_10_items_of_the_week():
    return get_all_items()[:10]

def export_to_frontend(item: ChemrxivItem, questions: list[Question], file_name: str):
    return item.to_frontend_article(questions).export_to_toml(file_name)


