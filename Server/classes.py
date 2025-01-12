import os
import pickle
from typing import Optional
from pypdf import PdfReader
import requests
import json

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

class Article:
    def __init__(self, article_data: Dict[str, Any]):
        self.article_data: Dict[str, Any] = article_data
        self.title: str = self.article_data.get('title', '')
        self.description: str = self.article_data.get('description', '')
        self.body: str = self.article_data.get('body', '')
        self.image: Image = Image(self.article_data.get('image', {}))
        self.authors: List[Author] = [Author(author_data) for author_data in self.article_data.get('authors', [])]
        self.professor: Professor = Professor(self.article_data.get('professor', {}))
        self.createdAt: str = self.article_data.get('createdAt', '')
        self.publishedAt: str = self.article_data.get('publishedAt', '')
        self.readingTime: int = self.article_data.get('readingTime', 0)
        self.updatedAt: Optional[str] = self.article_data.get('updatedAt', None)
        self.lastUpdatedAt: Optional[str] = self.article_data.get('lastUpdatedAt', None)

class Image:
    def __init__(self, image_data: Dict[str, Any]):
        self.image_data: Dict[str, Any] = image_data
        self.url: str = self.image_data.get('url', '')
        self.alt: str = self.image_data.get('alt', '')
        self.caption: str = self.image_data.get('caption', '')

class Professor:
    def __init__(self, professor_data: Dict[str, Any]):
        self.professor_data: Dict[str, Any] = professor_data
        self.name: str = self.professor_data.get('name', '')
        self.professorBio: str = self.professor_data.get('professorBio', '')
        self.slug: str = self.professor_data.get('slug', '')








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





