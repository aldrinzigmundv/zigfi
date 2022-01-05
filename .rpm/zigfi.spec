%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: zigfi
Summary: zigfi is an open-source stocks, commodities and cryptocurrencies price monitoring CLI app, written fully in Rust, where you can organize assets you&#x27;re watching easily into watchlists.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: AGPLv3+
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/aldrinzigmundv/zigfi

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
