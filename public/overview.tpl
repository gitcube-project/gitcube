@extends('profile.tpl')


@section('overview')
<div class="ui top attached tabular menu">
    <a class="active item" href="#">Overview</a>
    <a class="item" href="#">Repositories</a>
    <a class="item" href="#">Stars</a>
    <a class="item" href="#">Followers</a>
    <a class="item" href="#">Following</a>
</div>
<div class="ui bottom attached active tab segment">
    <div class="ui two column grid">
        <div class="row">
            <div class="column">
                <div class="ui fluid card">
                    <div class="content">
                        <a class="header" href="#">Mocores</a>
                        <div class="description">
                            <p>A distributed, high-scale computing system</p>
                        </div>
                    </div>
                    <div class="extra content">
                        <span class="left floated">
                            <a href="#" class="item">
                                <img src="assets/octicons-8.1.0/package/build/svg/star.svg" height="17"/>
                                Star
                            </a>
                            <a href="#" class="item">
                                <img src="assets/octicons-8.1.0/package/build/svg/git-branch.svg" height="17"/>
                                Fork
                            </a>
                        </span>
                    </div>
                </div>
            </div>
            <div class="column">
                <div class="ui fluid card">
                    <div class="content">
                        <a class="header" href="#">Minecase</a>
                        <div class="description">
                            <p>A distributed minecraft server in dotnet core</p>
                        </div>
                    </div>
                    <div class="extra content">
                        <span class="left floated">
                            <a href="#" class="item">
                                <img src="assets/octicons-8.1.0/package/build/svg/star.svg" height="17"/>
                                Star
                            </a>
                            <a href="#" class="item">
                                <img src="assets/octicons-8.1.0/package/build/svg/git-branch.svg" height="17"/>
                                Fork
                            </a>
                        </span>
                    </div>
                </div>
            </div>
        </div>
        <div class="row">
            <div class="column"></div>
            <div class="column"></div>
        </div>
        <div class="row">
            <div class="column"></div>
            <div class="column"></div>
        </div>
    </div>
    <div id="cal-heatmap"></div>
</div>


@endsection

@section('cal-heatmap')
<script type="text/javascript">
	var cal = new CalHeatMap();
	cal.init({ itemSelector: "cal-heatmap"});
</script>
@endsection