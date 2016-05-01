function searchResultDetails(documentId){
	alert(documentId);
}

function similarDocuments(documentId){
	alert(documentId);
}


function createSearchResultsItem(searchResult){
	var result='<div class="search-results-item">';
	result+='\n<div class="search-result">';

	result+='\n<div class="search-result-info">';
	result+='\n<p class="search-result-name">'+searchResult.name+'</p>';
	result+='\n</div>';


	result+='\n<div class="search-result-buttons">';
	result+='\n<a href="#" onclick="searchResultDetails(\''+searchResult.documentId+'\')" class="search-result-button">More</a>';
	result+='\n<a href="#" onclick="similarDocuments(\''+searchResult.documentId+'\')" class="search-result-button">Similar</a>';
	result+='\n</div>';

	result+='\n</div>';
	result+='\n</div>';
	return result;
}

function refreshSearchResults(searchResults){
	$('#search-results').html("");
	var currentRow;
	var currentRowItemsCount = 0;
	for (var index in searchResults) {

		$('#search-results').append($(createSearchResultsItem(searchResults[index])));
		currentRowItemsCount++;
		if (currentRowItemsCount >= 4){
			currentRowItemsCount = 0;
		}
	}
}

function submitSearchQuery(searchForm){
	$.ajax({
		url: "http://"+server_adress+":"+server_port+"/subtitles/search",
		data: $(searchForm).serialize(),
		success: processSearchQueryResults,
		dataType: "json"
	});
	return false;
}

function processSearchQueryResults(results){
	refreshSearchResults(results);
}
