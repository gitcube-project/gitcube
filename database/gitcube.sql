/*
Navicat MySQL Data Transfer

Source Server         : localhost
Source Server Version : 50721
Source Host           : localhost:3306
Source Database       : gitcube

Target Server Type    : MYSQL
Target Server Version : 50721
File Encoding         : 65001

Date: 2018-11-09 01:55:03
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for `orgs`
-- ----------------------------
DROP TABLE IF EXISTS `orgs`;
CREATE TABLE `orgs` (
  `uuid` varchar(36) NOT NULL,
  `name` varchar(64) NOT NULL,
  `description` varchar(512) DEFAULT NULL,
  PRIMARY KEY (`uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of orgs
-- ----------------------------

-- ----------------------------
-- Table structure for `repos`
-- ----------------------------
DROP TABLE IF EXISTS `repos`;
CREATE TABLE `repos` (
  `uuid` varchar(36) NOT NULL,
  `name` varchar(64) NOT NULL,
  `description` varchar(512) DEFAULT NULL,
  `owner_uuid` varchar(36) DEFAULT NULL,
  `status` int(11) DEFAULT NULL,
  `create_time` datetime DEFAULT NULL,
  `watches_num` int(11) NOT NULL DEFAULT '0',
  `stars_num` int(11) NOT NULL DEFAULT '0',
  `forks_num` int(11) NOT NULL DEFAULT '0',
  `is_private` int(11) NOT NULL DEFAULT '0',
  `is_fork` int(11) NOT NULL DEFAULT '0',
  `fork_uuid` varchar(36) DEFAULT NULL,
  PRIMARY KEY (`uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of repos
-- ----------------------------

-- ----------------------------
-- Table structure for `users`
-- ----------------------------
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `uuid` varchar(36) NOT NULL,
  `name` varchar(32) NOT NULL DEFAULT '',
  `fullname` varchar(32) NOT NULL,
  `email` varchar(128) DEFAULT NULL,
  `password` varchar(256) DEFAULT NULL,
  `status` int(11) DEFAULT NULL,
  PRIMARY KEY (`uuid`)
) ENGINE=MyISAM AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of users
-- ----------------------------
