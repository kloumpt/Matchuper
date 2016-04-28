function createSearchResultsItem(value){
	var result='<div class="search-results-item">';
	result+='\n<div class="search-result">';

	result+='\n<div class="search-result-info">';
	result+='\n<p class="search-result-name">'+value+'</p>';
	result+='\n</div>';

	result+='\n<div class="search-result-buttons">';
	result+='\n<a href="#" class="search-result-button">More</a>';
	result+='\n<a href="#" class="search-result-button">Similar</a>';
	result+='\n</div>';

	result+='\n</div>';
	result+='\n</div>';
	return result;
}

function refreshSearchResults(names){
	$('#search-results').html("");
	var currentRow;
	var currentRowItemsCount = 0;
	for (var name in names) {
		if (currentRowItemsCount === 0){
			currentRowItemsCount = 0;
			currentRow = $('<div class="search-results-row">');
			$('#search-results').append(currentRow);
		}

		$('#search-results').append($(createSearchResultsItem(names[name])));

		currentRowItemsCount++;
		if (currentRowItemsCount >= 4){
			currentRowItemsCount = 0;
		}
	}
}

function submitSearchQuery(searchForm){
	var names = ["Movie A", "Movie B", "Movie C","Movie D","Movie E","Movie F","Movie G", "Movie H", "Movie I", "Movie J","Movie K"];
	refreshSearchResults(names);
	return false;
}
