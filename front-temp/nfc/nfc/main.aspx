<%@ Page Language="C#" AutoEventWireup="true" CodeBehind="main.aspx.cs" Inherits="nfc.main" %>

<!DOCTYPE html>

<html xmlns="http://www.w3.org/1999/xhtml">
<head runat="server">
<meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title></title>
    <style type="text/css">
        .auto-style1 {
            width: 143px;
        }
    </style>
</head>
<body>
    <form id="form1" runat="server">
        <div>

            студент пришёл в аудиторию<br />
            <table style="width:100%;">
                <tr>
                    <td class="auto-style1">
                        <asp:Label ID="Label1" runat="server" Text="PK Студента"></asp:Label>
                    </td>
                    <td>
                        <asp:TextBox ID="in_PKStud" runat="server"></asp:TextBox>
                    </td>
                </tr>
                <tr>
                    <td class="auto-style1">
                        <asp:Label ID="Label2" runat="server" Text="PK Лекции"></asp:Label>
                    </td>
                    <td>
                        <asp:TextBox ID="in_PKLec" runat="server"></asp:TextBox>
                    </td>
                </tr>
                <tr>
                    <td class="auto-style1">
                        <asp:Label ID="Label3" runat="server" Text="Дата"></asp:Label>
                    </td>
                    <td>
                        <asp:TextBox ID="in_date" runat="server"></asp:TextBox>
                    </td>
                </tr>
            </table>
                    <asp:Button ID="Button1" runat="server" OnClick="Button1_Click" Text="Button" />
            студент покинул аудиторию
        </div>
        <table style="width:100%;">
            <tr>
                <td class="auto-style1">
            <asp:Label ID="Label4" runat="server" Text="PK Студента"></asp:Label>
                </td>
                <td>
            <asp:TextBox ID="TextBox4" runat="server"></asp:TextBox>
                </td>
            </tr>
            <tr>
                <td class="auto-style1">
            <asp:Label ID="Label5" runat="server" Text="PK Лекции"></asp:Label>
                </td>
                <td>
            <asp:TextBox ID="TextBox5" runat="server"></asp:TextBox>
                </td>
            </tr>
            <tr>
                <td class="auto-style1">
            <asp:Label ID="Label6" runat="server" Text="Дата"></asp:Label>
                </td>
                <td>
            <asp:TextBox ID="TextBox6" runat="server"></asp:TextBox>
                </td>
            </tr>
        </table>
    </form>
</body>
</html>
