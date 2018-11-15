/*
Navicat MySQL Data Transfer

Source Server         : localhost
Source Server Version : 50721
Source Host           : localhost:3306
Source Database       : gitcube

Target Server Type    : MYSQL
Target Server Version : 50721
File Encoding         : 65001

Date: 2018-11-16 02:16:41
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for `follow`
-- ----------------------------
DROP TABLE IF EXISTS `follow`;
CREATE TABLE `follow` (
  `follower_uuid` varchar(36) NOT NULL,
  `following_uuid` varchar(36) NOT NULL,
  PRIMARY KEY (`follower_uuid`,`following_uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of follow
-- ----------------------------

-- ----------------------------
-- Table structure for `org_user`
-- ----------------------------
DROP TABLE IF EXISTS `org_user`;
CREATE TABLE `org_user` (
  `org_uuid` varchar(36) NOT NULL,
  `user_uuid` varchar(36) NOT NULL,
  `role` int(11) NOT NULL,
  PRIMARY KEY (`org_uuid`,`user_uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of org_user
-- ----------------------------

-- ----------------------------
-- Table structure for `repo`
-- ----------------------------
DROP TABLE IF EXISTS `repo`;
CREATE TABLE `repo` (
  `uuid` varchar(36) NOT NULL,
  `name` varchar(64) NOT NULL,
  `description` varchar(512) DEFAULT NULL,
  `owner_uuid` varchar(36) DEFAULT NULL,
  `status` int(11) DEFAULT NULL,
  `create_time` datetime DEFAULT NULL,
  `is_private` int(11) NOT NULL DEFAULT '0',
  `is_fork` int(11) NOT NULL DEFAULT '0',
  `fork_uuid` varchar(36) DEFAULT NULL,
  PRIMARY KEY (`uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of repo
-- ----------------------------

-- ----------------------------
-- Table structure for `star`
-- ----------------------------
DROP TABLE IF EXISTS `star`;
CREATE TABLE `star` (
  `user_uuid` varchar(36) NOT NULL,
  `repo_uuid` varchar(36) NOT NULL,
  PRIMARY KEY (`user_uuid`,`repo_uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of star
-- ----------------------------

-- ----------------------------
-- Table structure for `user`
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user` (
  `uuid` varchar(36) NOT NULL,
  `name` varchar(32) NOT NULL DEFAULT '',
  `fullname` varchar(32) NOT NULL,
  `email` varchar(128) DEFAULT NULL,
  `password` varchar(256) NOT NULL,
  `is_block` int(11) NOT NULL,
  `avatar` varchar(256) NOT NULL,
  `type` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`uuid`)
) ENGINE=MyISAM AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of user
-- ----------------------------

-- ----------------------------
-- Table structure for `watch`
-- ----------------------------
DROP TABLE IF EXISTS `watch`;
CREATE TABLE `watch` (
  `watcher_uuid` varchar(36) NOT NULL,
  `repo_uuid` varchar(36) NOT NULL,
  PRIMARY KEY (`watcher_uuid`,`repo_uuid`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of watch
-- ----------------------------
