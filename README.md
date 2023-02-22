Often URLs contain another URL which has been encoded. So you click on the basic URL, the web site will decode the URL and probably redirect you to the second with a little extra tracking info for the web site as a bonus.

This little tool can help get to the inner url. Just say the main url is:
https://www.pleaseclick.com?url=.....encoded url&tracking_data=......

You could use: urlunwrap "^https://www.please" "url=(?P\<url\>.*)&tracking_data"
Paste the url into stdin and you will get the underlying url.

The first URL just acts as bit of sanity checking so the URL decode and regular expression
capture parts have more chance of working on valid data.
