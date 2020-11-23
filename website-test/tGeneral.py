import unittest
from selenium import webdriver
from selenium.webdriver.chrome.options import Options

class TestSnakeGame(unittest.TestCase):

    def test_basic(self):
        chrome_options = Options()
        chrome_options.add_argument('--headless')        
        self.browser = webdriver.Chrome()
        self.addCleanup(self.browser.quit)

        self.browser.get('http://localhost:8080/')
        assert 'Quite Cool Snake Game by Rob Purser' in self.browser.title
        
        section = self.browser.find_element_by_id('titlesection')
        assert 'not really that cool' in section.text
        
        section = self.browser.find_element_by_id('gamesection')
        assert 'The Destination' in section.text
        
        section = self.browser.find_element_by_id('backgroundsection')
        assert 'The Journey' in section.text
if __name__ == '__main__':
    unittest.main()
