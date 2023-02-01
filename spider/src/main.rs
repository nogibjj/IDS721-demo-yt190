//create a web crawler that takes a team name and return the live scores
use scraper::{Html, Selector};

fn main() {
    let response = reqwest::blocking::get("https://www.espn.com/nba/scoreboard")
        .unwrap()
        .text()
        .unwrap();

    let document = Html::parse_document(&response);

    let ul_selector = Selector::parse("ul.ScoreboardScoreCell__Competitors").unwrap();

    let elements = document.select(&ul_selector).map(|x| x.inner_html());

    elements.for_each(|x| {
        let name_selector = Selector::parse("div.ScoreCell__TeamName").unwrap();
        let score_selector = Selector::parse("div.ScoreCell__Score").unwrap();
        // let date_selector = Selector::parse("div.Card__Header__Title__Wrapper>h3").unwrap();
        // let mvp_selector = Selector::parse("div.Scoreboard__Column>a").unwrap();
        let doc = Html::parse_document(&x);

        let mut name_iter = doc.select(&name_selector).map(|x| x.inner_html());
        let mut score_iter = doc.select(&score_selector).map(|x| x.inner_html());
        // let mut mvp_iter = doc.select(&mvp_selector).map(|x| x.inner_html());
        // let mut date_iter = doc.select(&date_selector).map(|x| x.inner_html());

        // let date = date_iter.next().unwrap();
        // let mvp1 = mvp_iter.next().unwrap();
        // let mvp2 = mvp_iter.next().unwrap();

        let team1 = name_iter.next().unwrap();
        let team2 = name_iter.next().unwrap();

        let score1 = score_iter.next().unwrap();
        let score2 = score_iter.next().unwrap();

        // println!("{}({})", mvp1, mvp2);
        println!("{}({}) vs {}({})", team1, score1, team2, score2);

        ()
    });
}

// <h3 class="lister-item-header">
//         <span class="lister-item-index unbold text-primary">1.</span>

//     <a href="/title/tt0111161/?ref_=adv_li_tt">The Shawshank Redemption</a>
//     <span class="lister-item-year text-muted unbold">(1994)</span>
// </h3>

// <section class="Scoreboard bg-clr-white flex flex-auto justify-between" id="401468861">
// <div class="Scoreboard__RowContainer flex flex-column flex-auto">
// <div class="Scoreboard__Row flex w-100 Scoreboard__Row__Main">
// <div style="min-width:320px" class="Scoreboard__Column flex-auto Scoreboard__Column--1 Scoreboard__Column--Score">
// <div class=""><div class="ScoreCell nba ScoreCell--md ScoreCell--post" tabindex="0" role="group">
// <a class="ScoreCell__Link" href="/nba/game/_/gameId/401468861" data-game-link="true">
// <div class="ScoreCell__Link__Event__Detail"><div class="ScoreCell__CompetitorDetails">
// <ul class="ScoreCell__Competitors"><li class="ScoreCell__Item ScoreCell__Item--away ScoreCell__Item--winner">
// <img alt="" class="Image Logo ScoreCell__Logo mr3 Logo__sm" data-mptype="image" src="https://a.espncdn.com/combiner/i?img=/i/teamlogos/nba/500/scoreboard/mil.png&amp;scale=crop&amp;cquality=40&amp;location=origin&amp;w=40&amp;h=40"><div class="ScoreCell__Team"><div class="ScoreCell__Truncate clr-gray-01 h5">
// <div class="ScoreCell__TeamName ScoreCell__TeamName--shortDisplayName truncate db">Bucks</div>
// </div><div class="ScoreCell__Score h5 clr-gray-01 fw-heavy tar">150</div>
// </div><svg aria-hidden="true" class="ScoreCell__WinnerIcon icon__svg" viewBox="0 0 24 24">
// <use xlink:href="#icon__arrow__winner_left"></use></svg></li>
// <li class="ScoreCell__Item ScoreCell__Item--home ScoreCell__Item--loser"><img alt="" class="Image Logo ScoreCell__Logo mr3 Logo__sm" data-mptype="image" src="https://a.espncdn.com/combiner/i?img=/i/teamlogos/nba/500/scoreboard/det.png&amp;scale=crop&amp;cquality=40&amp;location=origin&amp;w=40&amp;h=40">
// <div class="ScoreCell__Team"><div class="ScoreCell__Truncate clr-gray-01 h5"><div class="ScoreCell__TeamName ScoreCell__TeamName--shortDisplayName truncate db">
// Pistons</div></div>
// <div class="ScoreCell__Score h5 clr-gray-01 fw-heavy tar">130</div></div></li></ul></div>
// <div class="ScoreCell__Overview"><div class="ScoreCell__Time h9 clr-gray-01">Final</div></div></div></a></div>
// </div></div></div></div></section>
