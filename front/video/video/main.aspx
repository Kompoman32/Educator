<%@ Page Language="C#" AutoEventWireup="true" CodeBehind="main.aspx.cs" Inherits="video.main" %>

<!DOCTYPE html>

<html xmlns="http://www.w3.org/1999/xhtml">
<head runat="server">
<meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title></title>
    <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.1.1/jquery.min.js"></script>
    <script type="text/javascript" src="/scripts/jwplayer.js"></script>
    <!--<script src="/scripts/main.js"></script>-->
</head>
<body>
    <form id="form1" runat="server">
        <div>
            <label>Введите ID</label>
            <input type="text" id="User"/>
            <input type="button" id="findUser" value="Авторизироваться"/>

        </div>
        <div id="content">
            <div><input type="button" id="v1" value="Online-курс по Blockchain. Лекция 1. Что такое Биткоин"/></div>
            <div><input type="button" id="v2" value="Online-курс по Blockchain. Лекция 2. Принципы работы Биткоин"/></div>
            <div id="player">For player</div>
        </div>
    </form>
</body>
    <script>
        var urlNode = '<%=video.Shared.GetURLNode() %>';
        //$('#content').hide();
        $('#player').hide();
        //  Проверка юзера
        var nameUser = $('#User').value; 
        $('#findUser').click(function () {
            $.ajax({
                type: "POST",
                url: urlNode,
                data: { name: nameUser },
                success: function (data) {
                    if (data.findUser)
                        $('#content').show();
                    else
                        $('#content').hide();
                },
            });
        });
        //  Списки видео
        $('#v1').click(function () {
            vplayer('https://www.youtube.com/watch?v=WBf8FZjPPqw&list=PLhZQuknA7yUBt82ow8rEfw_G8tNZjt3qB');

        });
        $('#v2').click(function () {
            vplayer('https://www.youtube.com/watch?v=cVbiN_FdfB8&list=PLhZQuknA7yUBt82ow8rEfw_G8tNZjt3qB&index=2');
        });
        //  настройка видеоплеера
        var vplayer = function (url) {
            jwplayer("player").setup({
                file: url,
                image: "files/pic.jpg",
                width: "640",
                height: "480",
                controls: false,
                autostart: false,
                mute: false,
                stretching: "uniform",
                title: "hello world",
            });
            $('#player').show();
        }
    </script>
</html>
